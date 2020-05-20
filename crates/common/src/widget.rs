use crate::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Widget {
    Circle { x: f32, y: f32, radius: f32 },
}
