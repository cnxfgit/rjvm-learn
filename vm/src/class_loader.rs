use std::collections::HashMap;

use crate::class::ClassRef;

#[derive(Debug, Default)]
pub struct ClassLoader<'a> {
    classes_by_name: HashMap<String, ClassRef<'a>>,
}

#[allow(dead_code)]
impl<'a> ClassLoader<'a> {
    pub fn register_class(&mut self, class: ClassRef<'a>) {
        self.classes_by_name.insert(class.name.clone(), class);
    }

    pub fn find_class_by_name(&self, name: &str) -> Option<ClassRef<'a>> {
        self.classes_by_name.get(name).cloned()
    }
}
