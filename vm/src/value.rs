use crate::{
    abstract_object::{AbstractObject, ObjectKind},
    array::Array,
    object::{self, Object},
    vm_error::VmError,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Value<'a> {
    #[default]
    Uninitialized,
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Object(AbstractObject<'a>),
    Null,
}

pub fn expect_abstract_object_at<'a>(
    vec: &[Value<'a>],
    index: usize,
) -> Result<AbstractObject<'a>, VmError> {
    let value = vec.get(index);
    if let Some(Value::Object(object)) = value {
        Ok(object.clone())
    } else {
        Err(VmError::ValidationException)
    }
}

pub fn expect_concrete_object_at<'a>(
    vec: &[Value<'a>],
    index: usize,
) -> Result<impl Object<'a>, VmError> {
    let value = expect_abstract_object_at(vec, index)?;
    if value.kind() == ObjectKind::Object {
        Ok(value)
    } else {
        Err(VmError::ValidationException)
    }
}

pub fn expect_array_at<'a>(vec: &[Value<'a>], index: usize) -> Result<impl Array<'a>, VmError> {
    let value = expect_abstract_object_at(vec, index)?;
    if value.kind() == ObjectKind::Array {
        Ok(value)
    } else {
        Err(VmError::ValidationException)
    }
}

pub fn expect_int_at(vec: &[Value], index: usize) -> Result<i32, VmError> {
    let value = vec.get(index);
    if let Some(Value::Int(int)) = value {
        Ok(*int)
    } else {
        Err(VmError::ValidationException)
    }
}

pub fn expect_float_at(vec: &[Value], index: usize) -> Result<f32, VmError> {
    let value = vec.get(index);
    if let Some(Value::Float(float)) = value {
        Ok(*float)
    } else {
        Err(VmError::ValidationException)
    }
}

pub fn expect_double_at(vec: &[Value], index: usize) -> Result<f64, VmError> {
    let value = vec.get(index);
    if let Some(Value::Double(double)) = value {
        Ok(*double)
    } else {
        Err(VmError::ValidationException)
    }
}