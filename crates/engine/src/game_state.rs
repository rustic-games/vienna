#[derive(Debug)]
pub(super) struct GameState {
    pub(crate) pos_x: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self { pos_x: 0.0 }
    }
}
