#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use engine::Engine;

fn main() -> anyhow::Result<()> {
    println!("Hello, from runner!");

    let mut engine = Engine::default();

    for plugin in find_plugins_in_path("plugins")? {
        engine.register_plugin(&plugin)?;
    }

    engine.run()?;

    Ok(println!("success"))
}

/// Find all files ending in *.wasm within the given path.
///
/// Files with duplicate names are ignored. Even if two plugins reside in
/// different directories, if their names are equal, only the first one is added
/// to the list of plugins.
fn find_plugins_in_path(path: &str) -> anyhow::Result<Vec<String>> {
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

            if let Some(path) = path.to_str() {
                paths.push(path.to_owned());
                duplicates.insert(file.to_owned());
            }
        }
    }

    Ok(paths)
}
