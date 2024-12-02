use std::fmt::{self, Formatter};

use crate::{field_flags::FieldFlags, field_type::FieldType};


#[derive(Debug, PartialEq)]
pub struct ClassFileField {
    pub flags: FieldFlags,
    pub name: String,
    pub type_descriptor: FieldType,
    pub constant_value: Option<FieldConstantValue>,
    pub deprecated: bool,
}

impl fmt::Display for ClassFileField {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {}: {} constant {:?}{}",
            self.flags,
            self.name,
            self.type_descriptor,
            self.constant_value,
            if self.deprecated { " (deprecated)" } else { "" }
        )
    }
}


#[derive(Debug, PartialEq, strum_macros::Display)]
pub enum FieldConstantValue {
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    String(String),
}