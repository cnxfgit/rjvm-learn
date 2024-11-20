use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use crate::constant_pool::InvalidConstantPoolIndexError;

#[derive(Debug, PartialEq, Eq)]
pub enum ClassReaderError {
    InvalidClassData(String, Option<InvalidConstantPoolIndexError>),
    UnsupportedVersion(u16, u16),
    InvalidTypeDescriptor(String),
}

impl ClassReaderError {
    pub fn invalid_class_data(message: String) -> Self {
        ClassReaderError::InvalidClassData(message, None)
    }
}

impl Display for ClassReaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ClassReaderError::InvalidClassData(details, _) => {
                write!(f, "invalid class file: {details}")
            }
            ClassReaderError::UnsupportedVersion(major, minor) => {
                write!(f, "unsupported class file version {major}.{minor}")
            }
            ClassReaderError::InvalidTypeDescriptor(descriptor) => {
                write!(f, "invalid type descriptor: {descriptor}")
            }
        }
    }
}

impl Error for ClassReaderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ClassReaderError::InvalidClassData(_, Some(source)) => Some(source),
            _ => None,
        }
    }
}
