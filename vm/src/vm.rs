use crate::value::Value;

pub struct Vm<'a> {

    pub printed: Vec<Value<'a>>,
}