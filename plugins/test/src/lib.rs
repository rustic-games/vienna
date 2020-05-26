//! A Vienna plugin used to test its development progress.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    future_incompatible,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rustdoc,
    unused
)]
#![allow(
    clippy::float_arithmetic,
    clippy::multiple_crate_versions,
    clippy::implicit_return,
    clippy::shadow_reuse
)]

vienna::plugin!();

/// Details about the movement request from the `MovingCircle` widget.
///
/// These details are embedded in the `move` event it triggers.
#[derive(Debug, Copy, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
struct Movement {
    position: Option<(f32, f32)>,
    direction: Option<Direction>,
    speed: Speed,
}

/// Direction the `MovingCircle` widget wants to move in.
///
/// This is an attribute of the `move` event it triggers.
#[derive(Debug, Copy, Clone, Deserialize)]
#[allow(clippy::missing_docs_in_private_items)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Speed with which the `MovingCircle` widget wants to move.
///
/// This is an attribute of the `move` event it triggers.
#[derive(Debug, Copy, Clone, Deserialize)]
#[allow(clippy::missing_docs_in_private_items)]
enum Speed {
    Normal,
    Fast,
    Turbo,
}

/// Runs once when the plugin is registered with the engine.
fn init() -> Registration {
    let button = widget::new("red", widget::ButtonRectangle)
        .attribute("width", 160.0)
        .attribute("height", 40.0)
        .attribute("idle_color", Color::new(0.5, 0.2, 0.9, 1.0))
        .attribute("focus_color", Color::new(0.1, 1.0, 0.8, 1.0))
        .attribute("active_color", Color::new(0.3, 0.5, 0.6, 1.0))
        .position(250.0, 50.0);

    let circle = widget::new("my_circle", widget::MovingCircle)
        .attribute("radius", 100.0)
        .attribute("fill_color", Color::new(0.0, 0.0, 0.0, 1.0))
        .attribute("border_width", 10.0)
        .position(200.0, 200.0);

    Registration::new("test").widget(circle).widget(button)
}

/// Runs on every game tick.
fn run(sdk: &Sdk, state: &mut State, events: &[Event]) -> Result<()> {
    let window_dimensions = sdk.canvas().dimensions();
    let widget = state
        .get_widget_mut("my_circle")
        .ok_or_else(|| format_err!("unable to find widget"))?;

    for event in events {
        if let Some(movement) = event_to_movement("my_circle", event) {
            transform_widget(widget, movement, window_dimensions)
        }
    }

    Ok(())
}

/// Given a widget, and any movement details fetched from the widget events,
#[allow(
    clippy::cast_possible_truncation,
    clippy::as_conversions,
    clippy::cast_lossless
)]
fn transform_widget(
    widget: &mut widget::WidgetWithPosition,
    movement: Movement,
    // TODO: change to f32
    (x_max, y_max): (u16, u16),
) {
    let (x, y) = widget.coordinates();
    let state = widget.state_mut();

    if let widget::Kind::MovingCircle = state.kind() {
        let radius = match state.get("radius").and_then(Value::as_f64) {
            Some(value) => value as f32,
            None => todo!("logging"),
        };

        let diameter = radius * 2.0;

        let (x_max, y_max) = (x_max as f32, y_max as f32);

        let dv = match movement.speed {
            Speed::Normal => 1.0,
            Speed::Fast => 3.0,
            Speed::Turbo => 5.0,
        };

        let (dv_x, dv_y) = movement
            .position
            .unwrap_or_else(|| match movement.direction {
                Some(Direction::Up) => (0.0, -dv),
                Some(Direction::Left) => (-dv, 0.0),
                Some(Direction::Down) => (0.0, dv),
                Some(Direction::Right) => (dv, 0.0),
                None => (0.0, 0.0),
            });

        // min/max so that the circle cannot move off the canvas.
        let x = (x + dv_x).min(x_max - diameter).max(0.0);
        let y = (y + dv_y).min(y_max - diameter).max(0.0);
        widget.set_coordinates(x, y);
    }
}

/// Convert an event to a movement type, if applicable.
#[allow(clippy::shadow_same)]
fn event_to_movement(widget_name: &str, event: &Event) -> Option<Movement> {
    match event {
        // Ignore any events that don't belong to the requested widget.
        Event::Widget { name, .. } if name != widget_name => None,

        Event::Widget { event, .. } if event.name() == "move" => {
            let direction = event
                .attribute("direction")
                .cloned()
                .map(serde_json::from_value)?
                .ok()?;

            let speed = event
                .attribute("speed")
                .cloned()
                .map(serde_json::from_value)?
                .ok()?;

            Some(Movement {
                position: None,
                direction,
                speed,
            })
        }

        Event::Widget { event, .. } if event.name() == "drag" => {
            let x: f64 = event
                .attribute("x")
                .cloned()
                .map(serde_json::from_value)?
                .ok()?;

            let y: f64 = event
                .attribute("y")
                .cloned()
                .map(serde_json::from_value)?
                .ok()?;

            Some(Movement {
                position: Some((x as f32, y as f32)),
                direction: None,
                speed: Speed::Normal,
            })
        }

        // After a resize we need to make sure the circle still fits within the
        // canvas boundaries.
        Event::Widget { event, .. } if event.name() == "resized" => {
            let position = event
                .attribute("delta")
                .and_then(Value::as_f64)
                .map(|v| (-v as f32, -v as f32));

            Some(Movement {
                position,
                direction: None,
                speed: Speed::Normal,
            })
        }
        _ => None,
    }
}
