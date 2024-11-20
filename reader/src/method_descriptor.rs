use crate::field_type::FieldType;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MethodDescriptor {
    pub parameters: Vec<FieldType>,
    pub return_type: Option<FieldType>,
}