use crate::{abstract_object::AbstractObject, vm_error::VmError};

pub enum MethodCallFailed<'a> {
    InternalError(VmError),
    ExceptionThrown(JavaException<'a>),
}

#[derive(Debug, PartialEq)]
pub struct JavaException<'a>(pub AbstractObject<'a>);