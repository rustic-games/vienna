//! Helper methods to handle widgets in the engine.

use common::{
    widget::{ButtonRectangle, MovingCircle, Runtime, Widget},
    Component, Event, WidgetWithPosition,
};
use std::convert::TryFrom;

/// Take a list of widgets, and a list of input events, and run each widget with
/// the given events.
///
/// The resulting widget events are returned to the caller, and the global
/// widget state is updated if the widget changed its internal state.
pub(super) fn update(
    name: &str,
    widget_with_position: &mut WidgetWithPosition,
    input_events: &[Event],
) -> Vec<Event> {
    let mut all_widget_events = vec![];
    let state = widget_with_position.state().clone().into();
    let mut rt = runtime(&state);

    for event in input_events {
        let mut widget_events = rt
            .interact(event)
            .into_iter()
            .map(|event| Event::Widget {
                name: name.to_owned(),
                event,
            })
            .collect();

        all_widget_events.append(&mut widget_events);
    }

    // Store the updated widget state, since the `interact` action
    // might have modified it.
    *widget_with_position.state_mut() = rt.state();

    all_widget_events
}

/// Return the components for a given widget.
pub(super) fn components(widget: &Widget) -> Vec<Component> {
    runtime(widget).render()
}

/// Get the runtime implementation of a widget.
fn runtime(widget: &Widget) -> Box<dyn Runtime> {
    #[allow(clippy::match_wild_err_arm)]
    match &widget {
        Widget::MovingCircle(state) => match MovingCircle::try_from(state) {
            Ok(widget) => Box::new(widget),
            Err(_) => todo!("logging"),
        },
        Widget::ButtonRectangle(state) => match ButtonRectangle::try_from(state) {
            Ok(widget) => Box::new(widget),
            Err(_) => todo!("logging"),
        },
    }
}
