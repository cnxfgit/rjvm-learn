use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Base(BaseType),
    Object(String),
    Array(Box<FieldType>),
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldType::Base(base) => write!(f, "{base}"),
            FieldType::Object(class) => f.write_str(class),
            FieldType::Array(component_type) => write!(f, "{component_type}[]"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, strum_macros::Display)]
#[repr(u8)]
pub enum BaseType {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
}
