use std::path::{Path, PathBuf};

pub struct FileSystemClassPathEntry {
    base_directory: PathBuf,
}

impl FileSystemClassPathEntry {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, InvalidDirectoryError> {
        let mut base_directory = PathBuf::new();
        base_directory.push(path);

        if !base_directory.exists() || !base_directory.is_dir() {
            Err(InvalidDirectoryError {
                path: base_directory.to_string_lossy().to_string(),
            })
        } else {
            Ok(Self { base_directory })
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidDirectoryError {
    path: String,
}
