use core::alloc;
use std::{fmt::Debug, marker::PhantomData};

use bitfield_struct::bitfield;
use rjvm_reader::{
    field_type::{self, BaseType, FieldType},
    type_conversion::ToUsizeSafe,
};

use crate::{
    alloc_entry::{self, AllocEntry},
    array::Array,
    array_entry_type::ArrayEntryType,
    class::{Class, ClassId, ClassRef},
    object::Object,
    value::Value,
    vm_error::VmError,
};

#[derive(Clone, PartialEq)]
#[repr(transparent)]
pub struct AbstractObject<'a> {
    data: *mut u8,
    marker: PhantomData<&'a [u8]>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum GcState {
    Unmarked,
    Marked,
}

impl From<u64> for GcState {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Unmarked,
            1 => Self::Marked,
            _ => panic!("invalid value for GcState: {}", value),
        }
    }
}

impl From<GcState> for u64 {
    fn from(value: GcState) -> Self {
        value as u64
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ObjectKind {
    Object,
    Array,
}

impl From<u64> for ObjectKind {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Object,
            1 => Self::Array,
            _ => panic!("invalid value for GcState: {}", value),
        }
    }
}

impl From<ObjectKind> for u64 {
    fn from(value: ObjectKind) -> Self {
        value as u64
    }
}

#[bitfield(u64)]
#[derive(PartialEq, Eq)]
pub(crate) struct AllocHeader {
    #[bits(1)]
    pub(crate) kind: ObjectKind,

    #[bits(1)]
    pub(crate) state: GcState,

    #[bits(30)]
    identity_hash_code: i32,

    #[bits(32)]
    pub(crate) size: usize,
}

#[repr(transparent)]
struct ObjectHeader {
    class_id: ClassId,
}

struct ArrayHeader {
    elements_type: ArrayEntryType,
    length: u32,
}

const fn align_to_8_bytes(required_size: usize) -> usize {
    match required_size % 8 {
        0 => required_size,
        n => required_size + (8 - n),
    }
}

pub(crate) const ALLOC_HEADER_SIZE: usize = align_to_8_bytes(size_of::<AllocHeader>());
pub(crate) const OBJECT_HEADER_SIZE: usize = align_to_8_bytes(size_of::<ObjectHeader>());
pub(crate) const ARRAY_HEADER_SIZE: usize = align_to_8_bytes(size_of::<ArrayHeader>());

impl<'a> AbstractObject<'a> {
    pub(crate) fn size_of_object(class: &Class) -> usize {
        let fields_sizes: usize = 8 * class.num_total_fields;
        ALLOC_HEADER_SIZE + OBJECT_HEADER_SIZE + fields_sizes
    }

    pub(crate) fn size_of_array(length: usize) -> usize {
        ALLOC_HEADER_SIZE + ARRAY_HEADER_SIZE + length * 8
    }

    pub fn new_object(class: &Class<'a>, alloc_entry: AllocEntry) -> Self {
        Self::write_object_header(class, &alloc_entry);
        Self {
            data: alloc_entry.ptr,
            marker: PhantomData,
        }
    }

    fn write_object_header(class: &Class, alloc_entry: &AllocEntry) {
        unsafe {
            let next_ptr = Self::write_alloc_header(alloc_entry, ObjectKind::Object);
            std::ptr::write(
                next_ptr as *mut ObjectHeader,
                ObjectHeader { class_id: class.id },
            );
        }
    }

    pub fn new_array(
        elements_type: ArrayEntryType,
        array_length: usize,
        alloc_entry: &AllocEntry,
    ) -> Self {
        Self::write_array_header(elements_type, array_length, alloc_entry);
        Self {
            data: alloc_entry.ptr,
            marker: PhantomData,
        }
    }

    fn write_array_header(
        elements_type: ArrayEntryType,
        array_length: usize,
        alloc_entry: &AllocEntry,
    ) {
        unsafe {
            let next_ptr = Self::write_alloc_header(alloc_entry, ObjectKind::Array);
            std::ptr::write(
                next_ptr as *mut ArrayHeader,
                ArrayHeader {
                    elements_type,
                    length: array_length as u32,
                },
            );
        }
    }

    unsafe fn write_alloc_header(alloc_entry: &AllocEntry, kind: ObjectKind) -> *mut u8 {
        let next_ptr = alloc_entry.ptr as *mut AllocHeader;
        std::ptr::write(
            next_ptr,
            AllocHeader::new()
                .with_kind(kind)
                .with_state(GcState::Unmarked)
                .with_identity_hash_code(identity_hash_code(alloc_entry.ptr))
                .with_size(alloc_entry.alloc_size),
        );
        next_ptr.add(1) as *mut u8
    }

    pub fn alloc_header(&self) -> &AllocHeader {
        unsafe { &*(self.data as *const AllocHeader) }
    }

    pub fn kind(&self) -> ObjectKind {
        self.alloc_header().kind()
    }

    pub fn alloc_size(&self) -> usize {
        self.alloc_header().size()
    }
}

fn hash(data: u64) -> u64 {
    (data >> 32) ^ (data)
}

fn identity_hash_code(ptr: *mut u8) -> i32 {
    let hash = hash(ptr as u64);

    let hash = (hash & ((1 << 30) - 1)) as u32;

    unsafe { std::mem::transmute(hash) }
}

unsafe fn write_value(ptr: *mut u8, value: Value) {
    match value {
        Value::Int(int) => std::ptr::write(ptr as *mut i32, int),
        Value::Long(long) => std::ptr::write(ptr as *mut i64, long),
        Value::Float(float) => std::ptr::write(ptr as *mut f32, float),
        Value::Double(double) => std::ptr::write(ptr as *mut f64, double),
        Value::Uninitialized | Value::Null => std::ptr::write(ptr as *mut u64, 0),
        Value::Object(obj) => std::ptr::write(ptr as *mut AbstractObject, obj),
    }
}

unsafe fn read_value<'a>(ptr: *const u8, field_type: &FieldType) -> Value<'a> {
    match field_type {
        FieldType::Base(BaseType::Boolean)
        | FieldType::Base(BaseType::Byte)
        | FieldType::Base(BaseType::Char)
        | FieldType::Base(BaseType::Short)
        | FieldType::Base(BaseType::Int) => Value::Int(std::ptr::read(ptr as *const i32)),
        FieldType::Base(BaseType::Long) => Value::Long(std::ptr::read(ptr as *const i64)),
        FieldType::Base(BaseType::Float) => Value::Float(std::ptr::read(ptr as *const f32)),
        FieldType::Base(BaseType::Double) => Value::Double(std::ptr::read(ptr as *const f64)),
        FieldType::Object(_) | FieldType::Array(_) => match std::ptr::read(ptr as *const i64) {
            0 => Value::Null,
            _ => Value::Object(std::ptr::read(ptr as *const AbstractObject)),
        },
    }
}

unsafe fn read_value2<'a>(ptr: *const u8, field_type: &ArrayEntryType) -> Value<'a> {
    match field_type {
        ArrayEntryType::Base(BaseType::Boolean)
        | ArrayEntryType::Base(BaseType::Byte)
        | ArrayEntryType::Base(BaseType::Char)
        | ArrayEntryType::Base(BaseType::Short)
        | ArrayEntryType::Base(BaseType::Int) => Value::Int(std::ptr::read(ptr as *const i32)),
        ArrayEntryType::Base(BaseType::Long) => Value::Long(std::ptr::read(ptr as *const i64)),
        ArrayEntryType::Base(BaseType::Float) => Value::Float(std::ptr::read(ptr as *const f32)),
        ArrayEntryType::Base(BaseType::Double) => Value::Double(std::ptr::read(ptr as *const f64)),
        ArrayEntryType::Object(_) | ArrayEntryType::Array => {
            match std::ptr::read(ptr as *const i64) {
                0 => Value::Null,
                _ => Value::Object(std::ptr::read(ptr as *const AbstractObject)),
            }
        }
    }
}

// As objects

impl<'a> AbstractObject<'a> {
    fn object_header(&self) -> &ObjectHeader {
        unsafe {
            let ptr = self.data.add(ALLOC_HEADER_SIZE);
            let header_ptr = ptr as *const ObjectHeader;
            &*header_ptr
        }
    }

    pub(crate) unsafe fn ptr_to_field_value(&self, field_index: usize) -> *mut u8 {
        let preceding_fields_size = 8 * field_index;
        let offset = ALLOC_HEADER_SIZE + OBJECT_HEADER_SIZE + preceding_fields_size;
        self.data.add(offset)
    }
}

impl<'a> Object<'a> for AbstractObject<'a> {
    fn class_id(&self) -> ClassId {
        self.object_header().class_id
    }

    fn set_field(&self, index: usize, value: Value<'a>) {
        unsafe {
            let ptr = self.ptr_to_field_value(index);
            write_value(ptr, value);
        }
    }

    fn get_field(&self, object_class: ClassRef, index: usize) -> Value<'a> {
        let field = object_class.field_at_index(index).unwrap();
        unsafe {
            let ptr = self.ptr_to_field_value(index);
            read_value(ptr, &field.type_descriptor)
        }
    }
}

// As arrays

impl<'a> AbstractObject<'a> {
    fn array_header(&self) -> &ArrayHeader {
        unsafe {
            let ptr = self.data.add(ALLOC_HEADER_SIZE);
            let header_ptr = ptr as *const ArrayHeader;
            &*header_ptr
        }
    }

    pub(crate) unsafe fn ptr_to_array_element(&self, element_index: usize) -> *mut u8 {
        let entry_location = 8 * element_index;
        let offset = ALLOC_HEADER_SIZE + ARRAY_HEADER_SIZE + entry_location;
        self.data.add(offset)
    }
}

impl<'a> Array<'a> for AbstractObject<'a> {
    fn elements_type(&self) -> ArrayEntryType {
        self.array_header().elements_type.clone()
    }

    fn len(&self) -> u32 {
        self.array_header().length
    }

    fn set_element(&self, index: usize, value: Value<'a>) -> Result<(), VmError> {
        if index >= self.len().into_usize_safe() {
            Err(VmError::ArrayIndexOutOfBoundsException)
        } else {
            unsafe {
                let ptr = self.ptr_to_array_element(index);
                write_value(ptr, value);
            }
            Ok(())
        }
    }

    fn get_element(&self, index: usize) -> Result<Value<'a>, VmError> {
        if index >= self.len().into_usize_safe() {
            Err(VmError::ArrayIndexOutOfBoundsException)
        } else {
            unsafe {
                let ptr = self.ptr_to_array_element(index);
                Ok(read_value2(ptr, &self.elements_type()))
            }
        }
    }
}

impl<'a> Debug for AbstractObject<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} ptr {:#0x} size {}",
            self.kind(),
            self.data as usize,
            self.alloc_size(),
        )?;

        match self.kind() {
            ObjectKind::Object => write!(f, " class_id {}", self.class_id()),
            ObjectKind::Array => write!(
                f,
                " elements type {:?} len {}",
                self.elements_type(),
                self.len()
            ),
        }
    }
}
