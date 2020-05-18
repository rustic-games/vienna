use crate::{config, error, plugin::Handler};
use common::{Event, GameState};
use std::time::Instant;

#[derive(Debug)]
pub struct Updater {
    pub(crate) config: config::Updater,

    // `update_interval` is the minimum amount of time (in nanoseconds) that
    // needs to pass before we trigger a game state update.
    update_interval: u64,

    // `total_time` is the total accumulation of passed time (in nanoseconds).
    // This is a monotonically increasing value. The value is passed to the
    // update handler of the game, which can use this when needed.
    total_time: u64,

    // `last_step_timestamp` is the timestamp at the end of the last game step.
    last_step_timestamp: Instant,

    // `accumulated_time` is the total time available (in nanoseconds) for the
    // update handler to run.
    accumulated_time: u64,

    // The remaining accumulated time is used as a range between 0 and 1 to let
    // the renderer know how far along the updater is towards providing the next
    // update.
    pub(super) step_progress: f64,
}

impl Updater {
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn run(
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
    pub(crate) fn update_game_state(
        &mut self,
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
        let update_interval = 1_000_000_000 / config.updates_per_second;

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
        let mut updater: Updater = config::Updater::default().into();
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
