use log::debug;

use crate::{
    call_stack::CallStack,
    class::{self, ClassId, ClassRef},
    class_manager::{ClassManager, ResolvedClass},
    class_resolver_by_id::ClassByIdResolver,
    exception::MethodCallFailed,
    value::Value,
    vm_error::VmError,
};

pub struct Vm<'a> {
    class_manager: ClassManager<'a>,

    pub printed: Vec<Value<'a>>,
}

impl<'a> ClassByIdResolver<'a> for Vm<'a> {
    fn find_class_by_id(&self, class_id: ClassId) -> Option<ClassRef<'a>> {
        self.class_manager.find_class_by_id(class_id)
    }
}

impl<'a> Vm<'a> {
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
}
