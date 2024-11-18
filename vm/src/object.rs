use crate::class::ClassId;



pub trait Object <'a> {
    fn class_id(&self) -> ClassId;
}