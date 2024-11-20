use std::{fmt, fmt::Formatter};

use rjvm_reader::{class_access_flags::ClassAccessFlags, constant_pool::ConstantPool};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ClassId(u32);

impl fmt::Display for ClassId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ClassId {}


#[derive(Debug)]
pub struct Class<'a> {
    pub id: ClassId,
    pub name: String,

    pub source_file: Option<String>,
    pub constants: ConstantPool,
    pub flags: ClassAccessFlags,
    pub superclass: Option<ClassRef<'a>>,
}

pub type ClassRef<'a> = &'a Class<'a>;