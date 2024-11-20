use crate::{attribute::Attribute, exception_table::ExceptionTable, line_number_table::LineNumberTable, method_descriptor::MethodDescriptor, method_flags::MethodFlags};

#[derive(Debug, PartialEq)]
pub struct ClassFileMethod {
    pub flags: MethodFlags,
    pub name: String,
    pub type_descriptor: String,
    pub parsed_type_descriptor: MethodDescriptor,
    pub attributes: Vec<Attribute>,
    pub code: Option<ClassFileMethodCode>,
    pub deprecated: bool,
    pub thrown_exceptions: Vec<String>,
}

#[derive(Debug, Default, PartialEq)]
pub struct ClassFileMethodCode {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: ExceptionTable,
    pub line_number_table: Option<LineNumberTable>,

    pub attributes: Vec<Attribute>,
}