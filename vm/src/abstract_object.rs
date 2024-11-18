use std::{fmt::Debug, marker::PhantomData};

use bitfield_struct::bitfield;

use crate::{array::Array, array_entry_type::ArrayEntryType, class::ClassId, object::Object};

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

impl<'a> AbstractObject<'a> {
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

// As objects

impl<'a> AbstractObject<'a> {
    fn object_header(&self) -> &ObjectHeader {
        unsafe {
            let ptr = self.data.add(ALLOC_HEADER_SIZE);
            let header_ptr = ptr as *const ObjectHeader;
            &*header_ptr
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
}

impl<'a> Array<'a> for AbstractObject<'a> {
    fn elements_type(&self) -> ArrayEntryType {
        self.array_header().elements_type.clone()
    }

    fn len(&self) -> u32 {
        self.array_header().length
    }
}

impl<'a> Object<'a> for AbstractObject<'a> {
    fn class_id(&self) -> ClassId {
        self.object_header().class_id
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
