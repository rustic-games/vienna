mod engine;
mod error;

pub use engine::Engine;
pub use error::EngineError;

type Result<T> = std::result::Result<T, EngineError>;
