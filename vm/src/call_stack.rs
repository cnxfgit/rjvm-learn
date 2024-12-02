use rjvm_reader::{
    class_file_method::ClassFileMethodCode, method_flags::MethodFlags, type_conversion::ToUsizeSafe,
};
use typed_arena::Arena;

use crate::{
    abstract_object::AbstractObject,
    call_frame::CallFrame,
    class_and_method::ClassAndMethod,
    stack_trace_element::StackTraceElement,
    value::Value,
    vm_error::VmError,
};

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
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_frame(
        &mut self,
        class_and_method: ClassAndMethod<'a>,
        receiver: Option<AbstractObject<'a>>,
        args: Vec<Value<'a>>,
    ) -> Result<CallFrameReference<'a>, VmError> {
        Self::check_receiver(&class_and_method, receiver.clone())?;
        let code = Self::get_code(&class_and_method)?;
        let locals = Self::prepare_loacls(code, receiver, args);
        let new_frame = self
            .allocator
            .alloc(CallFrame::new(class_and_method, locals));

        let reference = CallFrameReference(new_frame);
        self.frames.push(reference.clone());
        Ok(reference)
    }

    fn check_receiver(
        class_and_method: &ClassAndMethod,
        receiver: Option<AbstractObject<'a>>,
    ) -> Result<(), VmError> {
        if class_and_method.method.flags.contains(MethodFlags::STATIC) {
            if receiver.is_some() {
                return Err(VmError::ValidationException);
            }
        } else if receiver.is_none() {
            return Err(VmError::NullPointerException);
        }

        Ok(())
    }

    fn get_code<'b>(
        class_and_method: &'b ClassAndMethod,
    ) -> Result<&'b ClassFileMethodCode, VmError> {
        if class_and_method.is_native() {
            return Err(VmError::NotImplemented);
        }

        let code = &class_and_method.method.code.as_ref().unwrap();
        Ok(code)
    }

    fn prepare_loacls(
        code: &ClassFileMethodCode,
        receiver: Option<AbstractObject<'a>>,
        args: Vec<Value<'a>>,
    ) -> Vec<Value<'a>> {
        let mut locals: Vec<Value<'a>> = receiver
            .map(Value::Object)
            .into_iter()
            .chain(args.into_iter())
            .collect();
        while locals.len() < code.max_locals.into_usize_safe() {
            locals.push(Value::Uninitialized);
        }

        locals
    }

    pub fn pop_frame(&mut self) -> Result<(), VmError> {
        self.frames
            .pop()
            .map(|_| ())
            .ok_or(VmError::ValidationException)
    }

    pub fn get_stack_trace_elements(&self) -> Vec<StackTraceElement<'a>> {
        self.frames
            .iter()
            .rev()
            .map(|frame| frame.as_ref().to_stack_trace_element())
            .collect()
    }

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
