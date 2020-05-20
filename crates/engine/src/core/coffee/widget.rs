use coffee::graphics::{Color, Frame, Mesh, Point, Shape};
use common::Widget;

/// The Coffee core does not support high-DPI mode yet (retina screens).
///
/// See: <https://github.com/hecrj/coffee/issues/6>
///
/// The current way to deal with this works as follows:
///
/// When building the engine, the width and height of the window are defined by
/// the `game_state`.
///
/// When the `coffee` core is used, these values will be doubled so that the
/// window is the correct size on retina screens.
///
/// The actual size of the "canvas" is left unchanged. This allows the plugins
/// to use "points" as if they are pixels.
///
/// Then, in this function when we convert a widget to an actual graphic, we
/// double all pixel values.
pub(super) fn widget_to_graphic(frame: &mut Frame<'_>, widget: &Widget) {
    match widget {
        Widget::Circle { x, y, radius } => {
            let shape = Shape::Circle {
                center: Point::new(*x * 2.0, *y * 2.0),
                radius: *radius * 2.0,
            };

            let mut mesh = Mesh::new();
            mesh.fill(shape, Color::WHITE);
            mesh.draw(&mut frame.as_target());
        }
    }
}
