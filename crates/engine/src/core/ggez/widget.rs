use common::Widget;
use ggez::{graphics, nalgebra, Context, GameResult};

pub(super) fn widget_to_graphic(
    ctx: &mut Context,
    widget: &Widget,
) -> GameResult<impl graphics::Drawable> {
    match widget {
        Widget::Circle { x, y, radius } => graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            nalgebra::Point2::new(*x as f32, *y as f32),
            *radius as f32,
            2.0,
            graphics::WHITE,
        ),
    }
}
