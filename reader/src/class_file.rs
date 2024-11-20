use crate::{class_access_flags::ClassAccessFlags, class_file_field::ClassFileField, class_file_method::ClassFileMethod, class_file_version::ClassFileVersion, constant_pool::ConstantPool};



#[derive(Debug, Default)]
pub struct ClassFile {
    pub version: ClassFileVersion,
    pub constants: ConstantPool,
    pub flags: ClassAccessFlags,
    pub name: String,
    pub superclass: Option<String>,
    pub interface: Vec<String>,
    pub fields: Vec<ClassFileField>,
    pub methods: Vec<ClassFileMethod>,
    pub deprecated: bool,
    pub source_file: Option<String>,
}