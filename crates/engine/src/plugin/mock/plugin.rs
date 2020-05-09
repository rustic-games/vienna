use crate::error;
use crate::plugin::Runtime;

/// A mock plugin implementation
#[derive(Debug, Default)]
pub struct Plugin {
    pub(crate) runs: usize,
}

impl Runtime for Plugin {
    fn run(&mut self) -> Result<(), error::Runtime> {
        self.runs += 1;

        Ok(())
    }

    fn name(&self) -> &str {
        ""
    }

    fn as_mock(&mut self) -> Option<&mut Self> {
        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let mut mock = Plugin::default();
        mock.run().unwrap();
        mock.run().unwrap();

        assert_eq!(mock.runs, 2)
    }

    #[test]
    fn name() {
        assert_eq!(Plugin::default().name(), "")
    }
}
