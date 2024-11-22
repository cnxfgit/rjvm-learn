use std::str::Chars;

use crate::{class_reader_error::ClassReaderError, field_type::FieldType};
use ClassReaderError::InvalidTypeDescriptor;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MethodDescriptor {
    pub parameters: Vec<FieldType>,
    pub return_type: Option<FieldType>,
}

impl MethodDescriptor {
    pub fn parse(descriptor: &str) -> Result<MethodDescriptor, ClassReaderError> {
        let mut chars = descriptor.chars();
        match chars.next() {
            Some('(') => {
                let parameters = Self::parse_parameters(descriptor, &mut chars)?;
                if Some('(') == chars.next() {
                    let return_type = Self::parse_return_type(descriptor, &mut chars)?;
                    Ok(MethodDescriptor {
                        parameters,
                        return_type,
                    })
                } else {
                    Err(InvalidTypeDescriptor(descriptor.to_string()))
                }
            }
            _ => Err(InvalidTypeDescriptor(descriptor.to_string())),
        }
    }

    fn parse_parameters(
        descriptor: &str,
        chars: &mut Chars,
    ) -> Result<Vec<FieldType>, ClassReaderError> {
        let mut parameters = Vec::new();
        loop {
            match chars.clone().next() {
                Some(')') => return Ok(parameters),
                Some(_) => {
                    let param = FieldType::parse_from(descriptor, chars)?;
                    parameters.push(param);
                }
                None => return Err(InvalidTypeDescriptor(descriptor.to_string())),
            }
        }
    }

    fn parse_return_type(
        descriptor: &str,
        chars: &mut Chars,
    ) -> Result<Option<FieldType>, ClassReaderError> {
        match chars.clone().next() {
            Some('V') => Ok(None),
            Some(_) => {
                let return_type = Some(FieldType::parse_from(descriptor, chars)?);
                if chars.next().is_none() {
                    Ok(return_type)
                } else {
                    Err(InvalidTypeDescriptor(descriptor.to_string()))
                }
            }
            _ => Err(InvalidTypeDescriptor(descriptor.to_string())),
        }
    }
}
