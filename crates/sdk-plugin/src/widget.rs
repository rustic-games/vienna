use common::widget::Builder;
pub use common::{
    widget::{Kind, Kind::*},
    WidgetWithPosition,
};

pub fn new(name: impl Into<String>, kind: Kind) -> Builder {
    Builder::new(name, kind)
}
