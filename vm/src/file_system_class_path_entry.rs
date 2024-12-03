use std::path::{Path, PathBuf};

use crate::class_path_entry::{ClassLoadingError, ClassPathEntry};

#[derive(Debug)]
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

impl ClassPathEntry for FileSystemClassPathEntry {
    fn resolve(&self, class_name: &str) -> Result<Option<Vec<u8>>, ClassLoadingError> {
        let mut candidate = self.base_directory.clone();
        candidate.push(class_name);
        candidate.set_extension("class");
        if candidate.exists() {
            std::fs::read(candidate)
                .map(Some)
                .map_err(ClassLoadingError::new)
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidDirectoryError {
    path: String,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        class_path_entry::tests::{assert_can_find_class, assert_cannot_find_class},
        file_system_class_path_entry::{FileSystemClassPathEntry, InvalidDirectoryError},
    };

    #[test]
    fn directory_not_found() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("foobar");
        assert_eq!(
            InvalidDirectoryError {
                path: path.to_string_lossy().to_string()
            },
            FileSystemClassPathEntry::new(path).expect_err("should not have found directory")
        );
    }

    #[test]
    fn file_system_class_path_entry_works() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources");
        let entry = FileSystemClassPathEntry::new(path).expect("should find directory");

        assert_can_find_class(&entry, "rjvm/NumericTypes");
        assert_can_find_class(&entry, "rjvm/ControlFlow");
        assert_cannot_find_class(&entry, "rjvm/Foo");
    }
}
