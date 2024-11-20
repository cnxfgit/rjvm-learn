use core::fmt;
use std::{error::Error, f32::consts::E};

pub trait ClassPathEntry: fmt::Debug {
    fn resolve(&self, class_name: &str) -> Result<Option<Vec<u8>>, ClassLoadingError>;
}

#[derive(Debug)]
pub struct ClassLoadingError {
    message: String,
    source: Box<dyn Error>,
}

impl ClassLoadingError {
    pub fn new(error: impl Error + 'static) -> Self {
        Self {
            message: error.to_string(),
            source: Box::new(error),
        }
    }
}

impl fmt::Display for ClassLoadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ClassLoadingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}

// Test utilities used by multiple files
#[cfg(test)]
pub mod tests {
    use crate::class_path_entry::ClassPathEntry;

    pub fn assert_can_find_class(entry: &impl ClassPathEntry, class_name: &str) {
        let buf = entry
            .resolve(class_name)
            .expect("should have been able to read file")
            .expect("should have been able to find file");
        let magic_number =
            u32::from_be_bytes(buf[0..4].try_into().expect("file should have 4 bytes"));
        assert_eq!(0xCAFEBABE, magic_number);
    }

    pub fn assert_cannot_find_class(entry: &impl ClassPathEntry, class_name: &str) {
        assert!(entry
            .resolve(class_name)
            .expect("should not have had any errors")
            .is_none());
    }
}
