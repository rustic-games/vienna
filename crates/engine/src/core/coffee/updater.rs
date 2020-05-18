use crate::{config, error, plugin::Handler};
use common::{Event, GameState};

#[derive(Debug)]
pub struct Updater {
    pub(crate) config: config::Updater,

    /// A list of events that are currently active. This list is updated when
    /// new player input is received.
    ///
    /// When the updater runs, it drains all existing events.
    pub(crate) active_events: Vec<Event>,

    /// Returns true if the game should be closed.
    pub(crate) is_finished: bool,
}

impl Updater {
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn run(
        &mut self,
        state: &mut GameState,
        plugin_handler: &mut dyn Handler,
    ) -> Result<(), error::Updater> {
        plugin_handler.run_plugins(state, &self.active_events)?;

        self.active_events.clear();
        Ok(())
    }
}

impl From<config::Updater> for Updater {
    fn from(config: config::Updater) -> Self {
        Self {
            config,
            active_events: vec![],
            is_finished: false,
        }
    }
}
