use std::slice::Iter;

use thiserror::Error;

use crate::value::Value;

#[derive(Debug)]
pub struct ValueStack<'a> {
    stack: Vec<Value<'a>>,
}

#[derive(Error, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ValueStackError {
    #[error("trying to grow stack beyond maximum capacity")]
    MaximumCapacityReached,
    #[error("cannot pop from an empty stack")]
    CannotPopFromEmptyStack,
}

impl<'a> ValueStack<'a> {
    pub fn push(&mut self, value: Value<'a>) -> Result<(), ValueStackError> {
        if self.stack.len() < self.stack.capacity() {
            self.stack.push(value);
            Ok(())
        } else {
            Err(ValueStackError::MaximumCapacityReached)
        }
    }

    pub fn iter(&self) -> Iter<Value<'a>> {
        self.stack.iter()
    }
}
