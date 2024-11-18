use crate::abstract_object::AbstractObject;



#[derive(Debug, Default, Clone, PartialEq)]
pub enum Value<'a> {
    #[default]
    Uninitialized,
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Object(AbstractObject<'a>),
    Null,
}
