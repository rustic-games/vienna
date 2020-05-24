use common::{
    widget::{ButtonRectangle, MovingCircle, Runtime, Widget},
    Color, Event, Shape, WidgetWithPosition,
};
use ggez::{graphics, nalgebra, Context};
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

pub(super) fn render(ctx: &mut Context, widget: &Widget, (mut x, mut y): (f32, f32)) {
    let rt: Box<dyn Runtime> = match widget {
        Widget::MovingCircle(state) => Box::new(MovingCircle::try_from(state).expect("TODO")),
        Widget::ButtonRectangle(state) => Box::new(ButtonRectangle::try_from(state).expect("TODO")),
    };

    for component in rt.render() {
        let (x_rel, y_rel) = component.coordinates;

        x += x_rel;
        y += y_rel;

        let drawable = match component.shape {
            Shape::Circle { radius, color } => graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                nalgebra::Point2::new(x, y),
                radius.max(1.0),
                2.0,
                into_color(color),
            ),
            _ => todo!(),
        };

        graphics::draw(
            ctx,
            &drawable.expect("TODO"),
            graphics::DrawParam::default(),
        )
        .expect("TODO");
    }
}

fn into_color(color: Color) -> graphics::Color {
    let Color { r, g, b, a } = color;
    graphics::Color { r, g, b, a }
}
