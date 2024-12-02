use core::fmt;
use std::{alloc::Layout, fmt::Formatter, marker::PhantomData};

use log::{debug, info};
use rjvm_reader::{field_type::FieldType, type_conversion::ToUsizeSafe};

use crate::{
    abstract_object::{AbstractObject, AllocHeader, GcState, ObjectKind, ALLOC_HEADER_SIZE},
    alloc_entry::AllocEntry,
    array::Array,
    array_entry_type::ArrayEntryType,
    class::Class,
    class_resolver_by_id::ClassByIdResolver,
    object::Object,
    value::Value,
    vm_error::VmError,
};

struct MemoryChunk {
    memory: *mut u8,
    used: usize,
    capacity: usize,
}

impl fmt::Debug for MemoryChunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{address={:#0x}, used={}, capacity={}}}",
            self.memory as u64, self.used, self.capacity
        )
    }
}

impl MemoryChunk {
    fn new(capacity: usize) -> Self {
        let layout = Layout::from_size_align(capacity, 8).unwrap();
        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };

        debug!(
            "allocated memory chunk of size {} at {:#0x}",
            capacity, ptr as u64
        );

        MemoryChunk {
            memory: ptr,
            capacity,
            used: 0,
        }
    }

    fn alloc(&mut self, required_size: usize) -> Option<AllocEntry> {
        if self.used + required_size > self.capacity {
            return None;
        }

        assert_eq!(required_size % 8, 0);

        let ptr = unsafe { self.memory.add(self.used) };

        self.used += required_size;

        Some(AllocEntry {
            ptr,
            alloc_size: required_size,
        })
    }

    unsafe fn contains(&self, ptr: *const u8) -> bool {
        ptr >= self.memory && ptr <= self.memory.add(self.used)
    }

    fn reset(&mut self) {
        self.used = 0;

        unsafe {
            std::ptr::write_bytes(self.memory, 0, self.capacity);
        }
    }
}

pub struct ObjectAllocator<'a> {
    current: MemoryChunk,
    other: MemoryChunk,
    marker: PhantomData<&'a AbstractObject<'a>>,
}

impl<'a> ObjectAllocator<'a> {
    pub fn with_maximun_memory(max_size: usize) -> Self {
        let semi_space_capacity = max_size / 2;
        Self {
            current: MemoryChunk::new(semi_space_capacity),
            other: MemoryChunk::new(semi_space_capacity),
            marker: Default::default(),
        }
    }

    pub fn allocate_object(&mut self, class: &Class<'a>) -> Option<AbstractObject<'a>> {
        let size = AbstractObject::size_of_object(class);
        self.current
            .alloc(size)
            .map(|alloc_entry| AbstractObject::new_object(class, alloc_entry))
    }

    pub fn allocate_array(
        &mut self,
        elements_type: ArrayEntryType,
        length: usize,
    ) -> Option<AbstractObject<'a>> {
        let size = AbstractObject::size_of_array(length);
        self.current
            .alloc(size)
            .map(|alloc_entry| AbstractObject::new_array(elements_type, length, &alloc_entry))
    }

    pub unsafe fn do_garbage_collection(
        &mut self,
        roots: Vec<*mut AbstractObject<'a>>,
        class_resolver: &impl ClassByIdResolver<'a>,
    ) -> Result<(), VmError> {
        info!(
            "running gc; currently allocated memory = {}, gc roots count: {}",
            self.current.used,
            roots.len()
        );

        for root in roots.iter() {
            self.visit(*root, class_resolver)?;
        }

        Ok(())
    }

    unsafe fn visit(
        &mut self,
        object_ptr: *const AbstractObject<'a>,
        class_resolver: &impl ClassByIdResolver<'a>,
    ) -> Result<(), VmError> {
        let referred_object_ptr = *(object_ptr as *const *mut u8);
        assert!(self.current.contains(referred_object_ptr));
        let header = &mut *(referred_object_ptr as *mut AllocHeader);

        match header.state() {
            GcState::Unmarked => {
                header.set_state(GcState::Marked);

                if header.kind() == ObjectKind::Object {
                    self.visit_fields_of_object(&*object_ptr, class_resolver)?;
                } else {
                    self.visit_entries_of_array(&*object_ptr, class_resolver)?;
                }

                let new_address = self
                    .other
                    .alloc(header.size())
                    .map(|alloc_entry| {
                        std::ptr::copy_nonoverlapping(
                            referred_object_ptr,
                            alloc_entry.ptr,
                            header.size(),
                        );
                        alloc_entry.ptr
                    })
                    .expect("should have enough space in the other region");

                std::ptr::write(
                    referred_object_ptr.add(ALLOC_HEADER_SIZE) as *mut *mut u8,
                    new_address,
                );
            }
            GcState::Marked => {}
        }

        Ok(())
    }

    unsafe fn visit_fields_of_object(
        &mut self,
        object: &AbstractObject<'a>,
        class_resolver: &impl ClassByIdResolver<'a>,
    ) -> Result<(), VmError> {
        let class = class_resolver
            .find_class_by_id(object.class_id())
            .ok_or(VmError::ValidationException)?;

        debug!("should visit members of {object:?} of class {}", class.name);

        for (index, field) in class.all_fields().enumerate().filter(|(_, f)| {
            matches!(
                f.type_descriptor,
                FieldType::Object(_) | FieldType::Array(_)
            )
        }) {
            let field_value_ptr = object.ptr_to_field_value(index);
            debug!(
                "  should visit recursively field {} at offset {:#0x}",
                field.name, field_value_ptr as u64
            );

            if 0 == std::ptr::read(field_value_ptr as *const u64) {
                continue;
            }
            let field_object_ptr = field_value_ptr as *mut AbstractObject;
            self.visit(field_object_ptr, class_resolver)?;
        }

        Ok(())
    }

    unsafe fn visit_entries_of_array(
        &mut self,
        array: &AbstractObject<'a>,
        class_resolver: &impl ClassByIdResolver<'a>,
    ) -> Result<(), VmError> {
        match array.elements_type() {
            ArrayEntryType::Base(_) => Ok(()),
            ArrayEntryType::Object(_) => {
                for i in 0..array.len().into_usize_safe() {
                    let value = array.get_element(i);
                    match value {
                        Ok(Value::Object(array_element)) => {
                            debug!("  should visit recursively element at index {}", i);
                            self.visit(&array_element as *const AbstractObject, class_resolver)?;
                        }
                        Ok(Value::Null) => {}
                        _ => return Err(VmError::ValidationException),
                    }
                }
                Ok(())
            }
            ArrayEntryType::Array => {
                todo!("arrays of arrays are not supported yet")
            }
        }
    }
}
