use core::fmt;
use std::{collections::HashMap, fmt::Formatter};

use typed_arena::Arena;

use crate::{class::{Class, ClassId, ClassRef}, class_path::ClassPath, vm_error::VmError};

pub(crate) struct ClassManager<'a> {
    class_path: ClassPath,
    classes_by_id: HashMap<ClassId, ClassRef<'a>>,
    classes_by_name: HashMap<String, ClassRef<'a>>,

    arena: Arena<Class<'a>>,

    next_id: u32,

    current_class_loader: ClassLoader<'a>,
}

impl <'a> Default for ClassManager<'a> {
    fn default() -> Self {
        Self {
            class_path: Default::default(),
            classes_by_id: Default::default(),
            classes_by_name: Default::default(),
            arena: Arena::with_capacity(100),
            next_id: 1,
            current_class_loader: Default::default(),
        }
    }
}

impl <'a> fmt::Debug for ClassManager<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "class_manager={{loaded classes={}}}", self.arena.len())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ResolvedClass<'a> {
    AlreadyLoaded(ClassRef<'a>),
    NewClass(ClassesToInitialize<'a>)
}

impl<'a> ClassManager<'a> {
    pub fn get_or_resolve_class(&mut self, class_name: &str) -> Result<ResolvedClass<'a>, VmError> {
        if let Some(already_loaded_class) = self.find_class_by_name(class_name) {

        } else {
            self.resolve_and_load_class(class_name)
                .map(ResolvedClass::NewClass)
        }
    }
}
