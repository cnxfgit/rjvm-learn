use thiserror::Error;

use crate::value_stack::ValueStackError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum VmError {
    #[error("unexpected error loading class: {0}")]
    ClassLoadingError(String),

    #[error("null pointer exception")]
    NullPointerException,

    #[error("class not found: {0}")]
    ClassNotFoundException(String),

    #[error("method not found: {0}.{1}#{2}")]
    MethodNotFoundException(String, String, String),

    #[error("field not found: {0}.{1}")]
    FieldNotFoundException(String, String),

    #[error("validation exception - invalid class file")]
    ValidationException,

    #[error("arithmetic exception")]
    ArithmeticException,

    #[error("not yet implemented")]
    NotImplemented,

    #[error("array index out of bounds")]
    ArrayIndexOutOfBoundsException,

    #[error("class cast exception")]
    ClassCastException,
}

impl From<ValueStackError> for VmError {
    fn from(_: ValueStackError) -> Self {
        Self::ValidationException
    }
}
