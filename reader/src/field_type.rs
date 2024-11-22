use std::{fmt, str::Chars};

use crate::class_reader_error::ClassReaderError;
use itertools::Itertools;
use ClassReaderError::InvalidTypeDescriptor;

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Base(BaseType),
    Object(String),
    Array(Box<FieldType>),
}

impl FieldType {
    pub fn parse(type_desciptor: &str) -> Result<FieldType, ClassReaderError> {
        let mut chars = type_desciptor.chars();
        let descriptor = Self::parse_from(type_desciptor, &mut chars)?;
        match chars.next() {
            None => Ok(descriptor),
            Some(_) => Err(InvalidTypeDescriptor(type_desciptor.to_string())),
        }
    }

    pub(crate) fn parse_from(
        type_desciptor: &str,
        chars: &mut Chars,
    ) -> Result<FieldType, ClassReaderError> {
        let first_char = chars
            .next()
            .ok_or(InvalidTypeDescriptor(type_desciptor.to_string()))?;

        Ok(match first_char {
            'B' => FieldType::Base(BaseType::Byte),
            'C' => FieldType::Base(BaseType::Char),
            'D' => FieldType::Base(BaseType::Double),
            'F' => FieldType::Base(BaseType::Float),
            'I' => FieldType::Base(BaseType::Int),
            'J' => FieldType::Base(BaseType::Long),
            'S' => FieldType::Base(BaseType::Short),
            'Z' => FieldType::Base(BaseType::Boolean),
            'L' => {
                let class_name: String = chars.take_while_ref(|c| *c != ';').collect();
                match chars.next() {
                    Some(';') => FieldType::Object(class_name),
                    _ => return Err(InvalidTypeDescriptor(type_desciptor.to_string())),
                }
            }
            '[' => {
                let component_type = Self::parse_from(type_desciptor, chars)?;
                FieldType::Array(Box::new(component_type))
            }
            _ => return Err(InvalidTypeDescriptor(type_desciptor.to_string())),
        })
    }
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
