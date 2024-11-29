use std::{array, collections::HashMap, fmt::Error};

use log::{debug, info};
use rjvm_reader::type_conversion::ToUsizeSafe;
use typed_arena::Arena;

use crate::{
    abstract_object::{AbstractObject, ObjectKind},
    array::Array,
    array_entry_type::ArrayEntryType,
    call_stack::CallStack,
    class::{ClassId, ClassRef},
    class_manager::{ClassManager, ResolvedClass},
    class_resolver_by_id::ClassByIdResolver,
    exceptions::MethodCallFailed,
    gc::ObjectAllocator,
    native_methods_impl::array_copy,
    native_methods_registry::NativeMethodsRegistry,
    stack_trace_element::StackTraceElement,
    value::{self, Value},
    vm_error::VmError,
};

pub struct Vm<'a> {
    class_manager: ClassManager<'a>,

    object_allocator: ObjectAllocator<'a>,

    call_stack: Arena<CallStack<'a>>,

    statics: HashMap<ClassId, AbstractObject<'a>>,

    pub native_method_registry: NativeMethodsRegistry<'a>,

    throwable_call_stacks: HashMap<i32, Vec<StackTraceElement<'a>>>,

    pub printed: Vec<Value<'a>>,
}

pub const ONE_MEGABYTE: usize = 1024 * 1024;
const DEFAULT_MAX_MB_OF_MEMORY: usize = 100;
pub const DEFAULT_MAX_MEMORY: usize = 100 * ONE_MEGABYTE;
pub const DEFAULT_MAX_MEMORY_MB_STR: &str = const_format::formatcp!("{}", DEFAULT_MAX_MB_OF_MEMORY);

impl<'a> ClassByIdResolver<'a> for Vm<'a> {
    fn find_class_by_id(&self, class_id: ClassId) -> Option<ClassRef<'a>> {
        self.class_manager.find_class_by_id(class_id)
    }
}

impl<'a> Vm<'a> {
    pub fn new(max_memory: usize) -> Self {
        info!("Creating new VM with maximum memory {}", max_memory);
        let mut result = Self {
            class_manager: Default::default(),
            object_allocator: ObjectAllocator::with_maximun_memory(max_memory),
            call_stack: Arena::new(),
            statics: Default::default(),
            native_method_registry: Default::default(),
            throwable_call_stacks: Default::default(),
            printed: Vec::new(),
        };
        crate::native_methods_impl::register_natives(&mut result.native_method_registry);
        result
    }

    pub fn get_or_resolve_class(
        &mut self,
        stack: &mut CallStack<'a>,
        class_name: &str,
    ) -> Result<ClassRef<'a>, MethodCallFailed<'a>> {
        let class = self.class_manager.get_or_resolve_class(class_name)?;
        if let ResolvedClass::NewClass(classes_to_init) = &class {
            for class_to_init in classes_to_init.to_initialize.iter() {
                self.init_class(stack, class_to_init)?;
            }
        }

        Ok(class.get_class())
    }

    fn init_class(
        &mut self,
        stack: &mut CallStack<'a>,
        class_to_init: &ClassRef<'a>,
    ) -> Result<(), MethodCallFailed<'a>> {
        debug!("creating static instance of {}", class_to_init.name);

        Ok(())
    }

    pub fn get_class_by_id(&self, class_id: ClassId) -> Result<ClassRef<'a>, VmError> {
        self.find_class_by_id(class_id)
            .ok_or(VmError::ValidationException)
    }

    pub fn new_object(
        &mut self,
        call_stack: &mut CallStack<'a>,
        class_name: &str,
    ) -> Result<AbstractObject<'a>, MethodCallFailed<'a>> {
        let class = self.get_or_resolve_class(call_stack, class_name)?;
        Ok(self.new_object_of_class(class))
    }

    pub fn new_object_of_class(&mut self, class: ClassRef<'a>) -> AbstractObject<'a> {
        debug!("allocating new instance of {}", class.name);
        match self.object_allocator.allocate_object(class) {
            Some(object) => object,
            None => {
                self.run_garbage_collection()
                    .expect("could run garbage collection");
                self.object_allocator
                    .allocate_object(class)
                    .expect("cannot allocate object even after full garbage collection!")
            }
        }
    }

    pub fn new_array(
        &mut self,
        elements_type: ArrayEntryType,
        length: usize,
    ) -> AbstractObject<'a> {
        match self
            .object_allocator
            .allocate_array(elements_type.clone(), length)
        {
            Some(array) => array,
            None => {
                self.run_garbage_collection()
                    .expect("could run garbage collection");
                self.object_allocator
                    .allocate_array(elements_type, length)
                    .expect("cannot allocate array even after full garbage collection!")
            }
        }
    }

    pub fn clone_array(&mut self, value: Value<'a>) -> Result<Value<'a>, VmError> {
        match &value {
            Value::Object(array) if array.kind() == ObjectKind::Array => {
                let new_array =
                    self.new_array(array.elements_type(), array.len().into_usize_safe());
                array_copy(array, 0, &new_array, 0, array.len().into_usize_safe())?;
                Ok(Value::Object(new_array))
            }
            _ => Err(VmError::ValidationException),
        }
    }

    pub(crate) fn associate_stack_trace_with_throwable(
        &mut self,
        throwable: AbstractObject<'a>,
        call_stack: Vec<StackTraceElement<'a>>,
    ) {
        self.throwable_call_stacks
            .insert(throwable.identity_hash_code(), call_stack);
    }

    pub(crate) fn get_stack_trace_associated_with_throwable(
        &self,
        throwable: AbstractObject<'a>,
    ) -> Option<&Vec<StackTraceElement<'a>>> {
        self.throwable_call_stacks
            .get(&throwable.identity_hash_code())
    }

    pub fn run_garbage_collection(&mut self) -> Result<(), VmError> {
        let mut roots = vec![];
        roots.extend(
            self.statics
                .iter_mut()
                .map(|(_, object)| object as *mut AbstractObject<'a>),
        );
        roots.extend(self.call_stack.iter_mut().flat_map(|s| s.gc_roots()));

        unsafe {
            self.object_allocator
                .do_garbage_collection(roots, &self.class_manager)?
        }

        Ok(())
    }
}
