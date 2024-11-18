use typed_arena::Arena;

use crate::call_frame::CallFrame;

#[derive(Default)]
pub struct CallStack<'a> {
    allocator: Arena<CallFrame<'a>>,
}