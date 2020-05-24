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
    let widget = widget_with_position.widget().clone().into();

    let mut rt: Box<dyn Runtime> = match &widget {
        Widget::MovingCircle(state) => Box::new(MovingCircle::try_from(state).expect("TODO")),
        Widget::ButtonRectangle(state) => Box::new(ButtonRectangle::try_from(state).expect("TODO")),
    };

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
    *widget_with_position.widget_mut() = rt.state();

    all_widget_events
}

/// Return the components for a given widget.
pub(super) fn components(widget: &Widget) -> Vec<Component> {
    let rt: Box<dyn Runtime> = match widget {
        Widget::MovingCircle(state) => Box::new(MovingCircle::try_from(state).expect("TODO")),
        Widget::ButtonRectangle(state) => Box::new(ButtonRectangle::try_from(state).expect("TODO")),
    };

    rt.render()
}
