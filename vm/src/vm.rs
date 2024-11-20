use crate::{call_stack::CallStack, class::ClassRef, class_manager::ClassManager, exception::MethodCallFailed, value::Value};

pub struct Vm<'a> {
    class_manager: ClassManager<'a>,

    pub printed: Vec<Value<'a>>,

    
}

impl<'a> Vm<'a> {
    pub fn get_or_resolve_class(
        &mut self,
        stack: &mut CallStack<'a>,
        class_name: &str,
    ) -> Result<ClassRef<'a>, MethodCallFailed<'a>> {
        let class = self.class_manager.get_or_resolve_class(class_name)?;
        if let ResolvedClass::NewClass(classes_to_init) = &class {
            for class_to_init in classes_to_init {
                self.init_class(stack, class_to_init)
            }
        }

        Ok(class.get_class())
    }
}
