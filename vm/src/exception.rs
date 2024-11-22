use crate::{abstract_object::AbstractObject, vm_error::VmError};

pub enum MethodCallFailed<'a> {
    InternalError(VmError),
    ExceptionThrown(JavaException<'a>),
}

impl<'a> From<VmError> for MethodCallFailed<'a> {
    fn from(value: VmError) -> Self {
        Self::InternalError(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct JavaException<'a>(pub AbstractObject<'a>);
