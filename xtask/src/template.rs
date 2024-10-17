use std::path::{self};

use camino::Utf8PathBuf;
use serde::Serialize;

#[derive(Serialize)]
/// Additional context for path operations
pub struct PathContext {
    /// OS Specific separator
    pub separator: char,
}

impl PathContext {
    /// Create [`PathContext`]
    pub fn new() -> Self {
        Self {
            separator: path::MAIN_SEPARATOR,
        }
    }
}

#[derive(Serialize)]
/// Template context providing executable informations
pub struct Executable {
    /// Parent directory of executable
    pub directory: String,

    /// file name of executable (without extension)
    pub name: String,

    /// extension of executable
    pub extension: String,

    /// Full path of executable
    pub full_path: String,
}

impl Executable {
    /// Create [`Executable`] from full path
    pub fn new_from_path(path: Utf8PathBuf) -> Option<Self> {
        Some(Self {
            directory: path
                .parent()
                .map(|path| path.as_str())
                .unwrap_or("/")
                .to_string(),
            name: path.file_stem()?.to_string(),
            extension: path.extension()?.to_string(),
            full_path: path.into_string(),
        })
    }
}
