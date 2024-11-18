
#[derive(Debug, Clone,PartialEq, strum_macros::Display)]
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