//! The updater implementation for the coffee backend.

use crate::{config, error, plugin::Handler, widget};
use common::{Canvas, Event, GameState};

/// Handles updating the game state.
#[derive(Debug)]
pub struct Updater {
    /// The configuration of the updater.
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
    /// Update the game state.
    pub fn run(
        &mut self,
        state: &mut GameState,
        canvas: Canvas,
        plugin_handler: &mut dyn Handler,
    ) -> Result<(), error::Updater> {
        let mut widget_events = vec![];
        let input_events = &self.active_events;

        for (name, widget) in state.widgets_mut() {
            widget_events.append(&mut widget::update(name, widget, input_events))
        }

        plugin_handler.run_plugins(state, canvas, &widget_events)?;

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
