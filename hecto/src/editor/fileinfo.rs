use std::{fmt::Display, path::PathBuf};

#[derive(Clone, Default)]
pub struct FileInfo {
    pub path: Option<PathBuf>,
}

impl FileInfo {
    pub fn from(file_name: &str) -> Self {
        Self {
            path: Some(PathBuf::from(file_name)),
        }
    }
}

impl Display for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self
            .path
            .as_ref()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("[No Name]");
        write!(f, "{name}")
    }
}
