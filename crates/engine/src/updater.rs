use crate::{config, error, plugin::Handler};
use common::{Event, GameState};
use std::time::Instant;

// We'll define the `Nanoseconds` alias to make it easier to reason about
// numbers representing a timestamp.
type Nanoseconds = u64;

// This is a convenience constant, to make the rest of the code a bit easier to
// parse.
#[allow(non_upper_case_globals)]
const nanoseconds_per_second: u64 = 1_000_000_000;

#[derive(Debug)]
pub(super) struct Updater {
    pub config: config::Updater,

    // `update_interval` is the minimum amount of time (in nanoseconds) that
    // needs to pass before we trigger a game state update. This is a fixed
    // delta, to give us a predictable game simulation, and decouple our
    // simulation from the capabilities of the host in terms of rendering
    // performance.
    //
    // Think of it like this: after every frame render, we've given ourselves
    // some time to perform game state updates. We'll perform those updates at
    // the interval defined here, and we'll continue those updates for as long
    // as we don't have to render the next frame.
    update_interval: Nanoseconds,

    // `total_time` is the total accumulation of passed time (in nanoseconds).
    // This is a monotonically increasing value. The value is passed to the
    // update handler of the game, which can use this when needed.
    //
    // This uses `u64` as the storage type, using nanoseconds as the unit of
    // measurement, a single game session can run more than 500 years before we
    // get an integer overflow.
    total_time: Nanoseconds,

    // `last_step_timestamp` is the timestamp at the end of the last game step,
    // represented as an `Instant`. This value is updated after each game step,
    // allowing us to determine how long the last step took, and how much time
    // we have to run our update handler.
    last_step_timestamp: Instant,

    // `accumulated_time` is the total time available (in nanoseconds) for the
    // update handler to run. It is based off of the `current_time` value. After
    // each update step, we subtract the `delta_time` from the remaining
    // `accumulated_time`.
    //
    // When the accumulated time falls below the delta time, we render another
    // frame to the screen, and send the remaining accumulated time to the
    // render handler. We do this, so that the renderer can figure out how much
    // time there was left between the last game update and the next.
    //
    // Say for example that we moved to position X = 10 on the last update, and
    // the following is true:
    //
    // * we move 1X per update
    // * we update the game state 100 times per second (so we need 10
    //   milliseconds per update)
    // * our `accumulated_time` has 5 milliseconds remaining (remember, we
    //   _need_ 10 milliseconds to update the game, so the last 5 milliseconds
    //   are kept around)
    //
    // we now know that if we had 10 milliseconds remaining, the character
    // would've moved to X = 11. But since we only had 5 milliseconds left, the
    // character position wasn't updated in the last cycle. However, as soon as
    // we add 5 more milliseconds to our accumulator in the next cycle, it will
    // move to that X = 11 position.
    //
    // So, instead of rendering our character as "stopped" on X = 10, we'll
    // instead interpolate that we were at X = 10 in the last update, we would
    // have moved to X = 11 if we had 10 more milliseconds, but we only have 5
    // milliseconds left, which is 50% of a full movement, so we'll render the
    // character at X = 10 + 0.5 = 10.5.
    //
    // If, during the next update cycle, the character is moved to X = 11, we
    // can render the character there, and we've had three frames, the first one
    // rendering the character at position 10, the second frame at
    // 10.5, and the third at 11.
    //
    // If, however, it turns out the player instead instructed the character to
    // stop after the first frame (when the game still had the character
    // positioned at X = 10), we'll have to move the character back on the
    // screen. This will cause a (mostly unnoticeable) "stutter", but the fact
    // is that most of the time, the character would have ended up at X = 11,
    // making it a worthy trade off to have a once-every-while bad
    // interpolation, instead of constantly stuttering images due to not
    // interpolating the remaining accumulated update time every cycle.
    accumulated_time: Nanoseconds,

    // The remaining accumulated time is used as a range between 0 and 1 to let
    // the renderer know how far along the updater is towards providing the next
    // update.
    pub(super) step_progress: f64,
}

impl Updater {
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub(super) fn run(
        &mut self,
        state: &mut GameState,
        events: &[Event],
        plugin_handler: &mut dyn Handler,
    ) -> Result<(), error::Updater> {
        let last_step_duration = self.last_step_timestamp.elapsed();
        self.accumulated_time += last_step_duration.as_nanos() as u64;
        self.last_step_timestamp = Instant::now();

        // We check if there's enough time accumulated to actually
        // update a single game update. The required available time
        // depends on the configured updates per second.
        while self.accumulated_time >= self.update_interval {
            self.update_game_state(state, events, plugin_handler)?;

            self.accumulated_time -= self.update_interval;
            self.total_time += self.update_interval;
        }

        // The remaining accumulated time is used as a range between 0 and 1 to
        // let the renderer know how far along the updater is towards providing
        // the next update.
        self.step_progress = self.accumulated_time as f64 / self.update_interval as f64;

        Ok(())
    }

    #[allow(clippy::unused_self)]
    fn update_game_state(
        &self,
        state: &mut GameState,
        events: &[Event],
        plugin_handler: &mut dyn Handler,
    ) -> Result<(), error::Updater> {
        plugin_handler
            .run_plugins(state, events)
            .map_err(Into::into)
    }
}

impl From<config::Updater> for Updater {
    fn from(config: config::Updater) -> Self {
        let update_interval = nanoseconds_per_second / config.updates_per_second;

        Self {
            config,
            update_interval,
            total_time: 0,
            last_step_timestamp: Instant::now(),
            accumulated_time: 0,
            step_progress: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn update_game_state() {
        let mut state = GameState::default();
        let updater: Updater = config::Updater::default().into();
        let mut handler = crate::plugin::mock::Manager::default();
        handler.register_plugin(&mut state, Path::new("")).unwrap();

        updater
            .update_game_state(&mut state, &[], &mut handler)
            .unwrap();
        updater
            .update_game_state(&mut state, &[], &mut handler)
            .unwrap();

        assert_eq!(handler.as_mock().unwrap().plugins[0].runs, 2);
    }
}
