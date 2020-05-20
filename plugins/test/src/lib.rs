use std::collections::HashSet;

vienna::load!();

#[derive(Debug, Copy, Clone)]
struct Movement {
    direction: Direction,
    speed: Speed,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
enum Speed {
    Normal,
    Fast,
    Turbo,
}

fn init() -> Registration {
    Registration::new("test").widget(
        "circle",
        Widget::Circle {
            x: 200.0,
            y: 200.0,
            radius: 100.0,
        },
    )
}

fn run(sdk: &Sdk, state: &mut State, events: &[Event]) -> Result<()> {
    let window_dimensions = sdk.canvas().dimensions();
    let widget = state.get_widget_mut("circle").unwrap();

    for event in events {
        for movement in event_to_movements(&event) {
            transform_widget(widget, movement, window_dimensions)
        }
    }

    Ok(())
}

fn transform_widget(widget: &mut Widget, movement: Movement, (x_max, y_max): (u16, u16)) {
    match widget {
        Widget::Circle { x, y, radius } => {
            let (x_max, y_max) = (x_max as f32, y_max as f32);
            let (x_old, y_old, radius) = (*x, *y, *radius);

            let dv = match movement.speed {
                Speed::Normal => 1.0,
                Speed::Fast => 3.0,
                Speed::Turbo => 5.0,
            };

            let (x_new, y_new) = match movement.direction {
                Direction::Up => (x_old, radius.max(y_old - dv)),
                Direction::Left => (radius.max(x_old - dv), y_old),
                Direction::Down => (x_old, (y_max - radius).min(y_old + dv)),
                Direction::Right => ((x_max - radius).min(x_old + dv), y_old),
            };

            *x = x_new;
            *y = y_new;
        }
    }
}

fn event_to_movements(event: &Event) -> Vec<Movement> {
    match event {
        Event::Keyboard(keys) => {
            let speed = keys_to_speed(&keys);
            keys.iter()
                .copied()
                .filter_map(key_to_direction)
                .map(|direction| Movement { direction, speed })
                .collect()
        }
        _ => vec![],
    }
}

fn keys_to_speed(keys: &HashSet<Key>) -> Speed {
    match () {
        _ if keys.contains(&Key::Shift) => Speed::Fast,
        _ if keys.contains(&Key::Ctrl) => Speed::Turbo,
        _ => Speed::Normal,
    }
}

fn key_to_direction(key: Key) -> Option<Direction> {
    match key {
        Key::W => Some(Direction::Up),
        Key::A => Some(Direction::Left),
        Key::S => Some(Direction::Down),
        Key::D => Some(Direction::Right),
        _ => None,
    }
}
