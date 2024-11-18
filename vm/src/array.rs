use crate::array_entry_type::ArrayEntryType;



pub trait Array<'a> {
    fn elements_type(&self) -> ArrayEntryType;

    fn len(&self) -> u32;
}