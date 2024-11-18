
use thiserror::Error;


#[derive(Debug, Error, PartialEq, Eq)]
pub enum VmError {
    #[error("not yet implemented")]
    NotImplemented,
}