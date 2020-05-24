vienna::plugin!();

/// Details about the movement request from the `MovingCircle` widget.
///
/// These details are embedded in the `move` event it triggers.
#[derive(Debug, Copy, Clone)]
struct Movement {
    direction: Option<Direction>,
    speed: Speed,
}

/// Direction the `MovingCircle` widget wants to move in.
///
/// This is an attribute of the `move` event it triggers.
#[derive(Debug, Copy, Clone, Deserialize)]
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
enum Speed {
    Normal,
    Fast,
    Turbo,
}

/// Runs once when the plugin is registered with the engine.
fn init() -> Registration {
    let circle = widget::new("my_circle", widget::MovingCircle)
        .attribute("radius", 100.0)
        .attribute("color", Color::new(0.0, 0.0, 0.0, 1.0))
        .position(200.0, 200.0);

    Registration::new("test").widget(circle)
}

/// Runs on every game tick.
fn run(sdk: &Sdk, state: &mut State, events: &[Event]) -> Result<()> {
    let window_dimensions = sdk.canvas().dimensions();
    let widget = state.get_widget_mut("my_circle").expect("TODO");

    for event in events {
        if let Some(movement) = event_to_movement("my_circle", &event) {
            transform_widget(widget, movement, window_dimensions)
        }
    }

    Ok(())
}

/// Given a widget, and any movement details fetched from the widget events,
fn transform_widget(
    widget: &mut widget::WidgetWithPosition,
    movement: Movement,
    // TODO: change to f32
    (x_max, y_max): (u16, u16),
) {
    let (x, y) = widget.coordinates();
    let state = widget.widget_mut();

    match state.kind() {
        widget::Kind::MovingCircle => {
            let radius = state.get("radius").expect("TODO").as_f64().expect("TODO") as f32;

            let (x_max, y_max) = (x_max as f32, y_max as f32);
            let (x_old, y_old, radius) = (x, y, radius);

            let dv = match movement.speed {
                Speed::Normal => 1.0,
                Speed::Fast => 3.0,
                Speed::Turbo => 5.0,
            };

            let (dv_x, dv_y) = match movement.direction {
                Some(Direction::Up) => (0.0, -dv),
                Some(Direction::Left) => (-dv, 0.0),
                Some(Direction::Down) => (0.0, dv),
                Some(Direction::Right) => (dv, 0.0),
                None => (0.0, 0.0),
            };

            // min/max so that the circle cannot move off the canvas.
            let x = (x_old + dv_x).min(x_max - radius).max(radius);
            let y = (y_old + dv_y).min(y_max - radius).max(radius);
            widget.set_coordinates(x, y);
        }
        _ => todo!(),
    }
}

fn event_to_movement(widget_name: &str, event: &Event) -> Option<Movement> {
    match event {
        // Ignore any events that don't belong to the requested widget.
        Event::Widget { name, .. } if name != widget_name => None,

        Event::Widget { name: _, event } if event.name() == "move" => {
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

            Some(Movement { direction, speed })
        }

        // After a resize we need to make sure the circle still fits within the
        // canvas boundaries.
        Event::Widget { name: _, event } if event.name() == "resized" => Some(Movement {
            direction: None,
            speed: Speed::Normal,
        }),
        _ => None,
    }
}
