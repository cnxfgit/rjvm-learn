use rjvm_reader::field_type::BaseType;

use crate::class::ClassId;



#[derive(PartialEq, Clone, Debug)]
#[repr(u8)]
pub enum ArrayEntryType {
    Base(BaseType),
    Object(ClassId),
    Array,    
}