use typed_arena::Arena;

use crate::{abstract_object::AbstractObject, call_frame::CallFrame};

#[derive(Default)]
pub struct CallStack<'a> {
    frames: Vec<CallFrameReference<'a>>,
    allocator: Arena<CallFrame<'a>>,
}

#[derive(Debug, Clone)]
pub struct CallFrameReference<'a>(*mut CallFrame<'a>);

impl<'a> AsRef<CallFrame<'a>> for CallFrameReference<'a> {
    fn as_ref(&self) -> &CallFrame<'a> {
        unsafe { self.0.as_ref().unwrap() }
    }
}

impl<'a> AsMut<CallFrame<'a>> for CallFrameReference<'a> {
    fn as_mut(&mut self) -> &mut CallFrame<'a> {
        unsafe { self.0.as_mut() }.unwrap()
    }
}

impl<'a> CallStack<'a> {
    pub fn gc_roots(&mut self) -> impl Iterator<Item = *mut AbstractObject<'a>> {
        let mut roots = vec![];
        roots.extend(
            self.frames
                .iter_mut()
                .flat_map(|frame| frame.as_mut().gc_roots()),
        );
        roots.into_iter()
    }
}
