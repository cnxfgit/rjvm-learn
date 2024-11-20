use thiserror::Error;


#[derive(Debug, PartialEq)]
pub enum ConstantPoolEntry {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    ClassReference(u16),
    StringReference(u16),
    FieldReference(u16, u16),
    MethodReference(u16, u16),
    InterfaceMethodReference(u16, u16),
    NameAndTypeDescriptor(u16, u16),
}

#[derive(Debug)]
enum ConstantPoolPhysicalEntry {
    Entry(ConstantPoolEntry),
    MultiByteEntryTombstone(),
}

#[derive(Debug, Default)]
pub struct ConstantPool {
    entries: Vec<ConstantPoolPhysicalEntry>,
}


#[derive(Error, Debug, PartialEq, Eq)]
#[error("invalid constant pool index: {index}")]
pub struct InvalidConstantPoolIndexError {
    pub index: u16,
}