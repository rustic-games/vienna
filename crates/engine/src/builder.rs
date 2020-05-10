use crate::error::Builder as Error;
use crate::plugin::{wasm, Handler};
use crate::Engine;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Default)]
pub struct Builder<'a> {
    plugin_paths: Vec<&'a str>,
}

impl<'a> Builder<'a> {
    pub fn with_plugin_path(mut self, path: &'a str) -> Self {
        self.plugin_paths.push(path);
        self
    }
}

impl<'a> Builder<'a> {
    pub fn build(self) -> Result<Engine> {
        let mut plugin_handler = Box::new(wasm::Manager::default());

        for path in &self.plugin_paths {
            for plugin in find_plugins_in_path(path)? {
                plugin_handler.register_plugin(&plugin)?;
            }
        }

        Ok(Engine {
            plugin_handler,
            ..Engine::default()
        })
    }
}

/// Find all files ending in *.wasm within the given path.
///
/// Files with duplicate names are ignored. Even if two plugins reside in
/// different directories, if their names are equal, only the first one is added
/// to the list of plugins.
fn find_plugins_in_path(path: &str) -> Result<Vec<PathBuf>> {
    use std::collections::HashSet;
    use std::ffi::OsStr;
    use walkdir::WalkDir;

    let mut paths = vec![];
    let mut duplicates = HashSet::new();

    for entry in WalkDir::new(path) {
        let entry = entry?;

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        if path.extension().and_then(OsStr::to_str) != Some("wasm") {
            continue;
        }

        if let Some(file) = path.file_name().and_then(OsStr::to_str) {
            if duplicates.contains(file) {
                continue;
            }

            paths.push(path.to_owned());
            duplicates.insert(file.to_owned());
        }
    }

    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod with_plugin_path {
        use super::*;

        #[test]
        fn works() {
            let builder = Builder::default();
            let builder = builder.with_plugin_path("foo");

            assert_eq!(builder.plugin_paths.get(0), Some(&"foo"));
        }
    }

    mod build {
        use super::*;
        use tempfile::NamedTempFile;

        #[test]
        fn without_paths() {
            let builder = Builder::default();

            assert!(builder.build().is_ok())
        }

        #[test]
        fn with_valid_path() {
            let file = NamedTempFile::new().expect("temporary file");

            let builder = Builder::default();
            let builder = builder.with_plugin_path(file.path().to_str().unwrap());

            assert!(builder.build().is_ok())
        }

        #[test]
        fn with_invalid_path() {
            let builder = Builder::default();
            let builder = builder.with_plugin_path("foo");

            let err = builder.build().unwrap_err();

            assert_eq!(
                err.to_string(),
                format!("inaccessible plugin `foo` (NotFound)")
            )
        }
    }
}
