//! Helper methods to handle widgets in the engine.

use common::{
    event,
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
    let coordinates = widget_with_position.coordinates();
    let state = widget_with_position.state().clone().into();
    let mut rt = runtime(&state);

    for event in input_events {
        for widget_event in widget_events(event.clone(), &*rt, widget_with_position, coordinates) {
            let mut widget_events = rt
                .interact(&widget_event)
                .into_iter()
                .map(|event| Event::Widget {
                    name: name.to_owned(),
                    event,
                })
                .collect();

            all_widget_events.append(&mut widget_events);
        }
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

/// Check whether the widget wants to know about a given event.
fn widget_events(
    event: Event,
    rt: &dyn Runtime,
    widget: &mut WidgetWithPosition,
    widget_coordinates: (f32, f32),
) -> Vec<Event> {
    let mut events = vec![];

    let mut handle_event = |kind: usize, button: event::MouseButton, pointer: (f32, f32)| {
        let (relative_coordinates, event) =
            handle_pointer_widget_bounds(rt, widget, widget_coordinates, pointer);

        if let Some(event) = event {
            events.push(Event::Input(event));
        }

        if let Some((x, y)) = relative_coordinates {
            let input = match () {
                _ if kind == 0 => event::Input::Pointer(x, y),
                _ if kind == 1 => event::Input::MouseClick { button, x, y },
                _ => return None,
            };

            return Some(Event::Input(input));
        }

        None
    };

    match event {
        Event::Input(event::Input::Pointer(x, y)) => {
            if let Some(event) = handle_event(0, event::MouseButton::Left /* dummy */, (x, y)) {
                events.push(event);
            }
        }

        Event::Input(event::Input::MouseClick { button, x, y }) => {
            if let Some(event) = handle_event(1, button, (x, y)) {
                events.push(event);
            }
        }

        event => events.push(event),
    }

    events
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

/// Takes a set of parameters to compute if the mouse cursor is within the
/// bounds of a widget.
///
/// It returns the relative coordinates from the widget's top-left position of
/// the cursor if it is within the bounds.
///
/// It also returns a blur/focus event if needed.
fn handle_pointer_widget_bounds(
    rt: &dyn Runtime,
    widget: &mut WidgetWithPosition,
    widget_coordinates: (f32, f32),
    pointer_coordinates: (f32, f32),
) -> (Option<(f32, f32)>, Option<event::Input>) {
    let (x_widget, y_widget) = widget_coordinates;
    let (x, y) = pointer_coordinates;

    let mut blur = || {
        if widget.focussed() {
            widget.blur();
            return (None, Some(event::Input::Blur));
        };

        (None, None)
    };

    // pointer is to the left or top of widget.
    if x < x_widget || y < y_widget {
        return blur();
    }

    let (widget_width, widget_height) = rt.dimensions();

    // pointer is to the right or bottom of widget.
    if x > x_widget + widget_width || y > y_widget + widget_height {
        return blur();
    }

    let x_relative = x - x_widget;
    let y_relative = y - y_widget;

    if !rt.is_within_bounds(x_relative, y_relative) {
        return blur();
    }

    let event = if widget.focussed() {
        None
    } else {
        widget.focus();
        Some(event::Input::Focus)
    };

    (Some((x_relative, y_relative)), event)
}
