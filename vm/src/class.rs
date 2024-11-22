use std::{fmt, fmt::Formatter};

use rjvm_reader::{
    class_access_flags::ClassAccessFlags, class_file::ClassFile, class_file_field::ClassFileField,
    class_file_method::ClassFileMethod, constant_pool::ConstantPool,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ClassId(u32);

impl fmt::Display for ClassId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ClassId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

#[derive(Debug)]
pub struct Class<'a> {
    pub id: ClassId,
    pub name: String,

    pub source_file: Option<String>,
    pub constants: ConstantPool,
    pub flags: ClassAccessFlags,
    pub superclass: Option<ClassRef<'a>>,
    pub interfaces: Vec<ClassRef<'a>>,
    pub fields: Vec<ClassFileField>,
    pub methods: Vec<ClassFileMethod>,
    pub first_field_index: usize,
    pub num_total_fields: usize,
}

pub type ClassRef<'a> = &'a Class<'a>;

impl<'a> Class<'a> {
    pub fn is_subclass_of(&self, base: ClassRef) -> bool {
        self.name == base.name
            || self
                .superclass
                .map_or(false, |superclass| superclass.is_subclass_of(base))
            || self.interfaces.iter().any(|intf| intf.is_subclass_of(base))
    }
}
