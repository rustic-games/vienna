use crate::{EngineError, Result};

#[derive(Default)]
pub struct Engine;

impl Engine {
    pub fn run(self) -> Result<()> {
        println!("Hello, from engine!");

        Err(EngineError::Unknown)
    }
}
