use std::f32::NAN;

use rjvm_reader::field_type::BaseType;

use crate::{
    abstract_object::AbstractObject,
    array::Array,
    array_entry_type::ArrayEntryType,
    call_stack::{self, CallStack},
    class,
    exceptions::MethodCallFailed,
    object::Object,
    value::Value,
    vm::Vm,
};

pub fn new_java_lang_string_object<'a>(
    vm: &mut Vm<'a>,
    call_stack: &mut CallStack<'a>,
    content: &str,
) -> Result<AbstractObject<'a>, MethodCallFailed<'a>> {
    let char_array: Vec<Value<'a>> = content
        .encode_utf16()
        .map(|c| Value::Int(c as i32))
        .collect();
    let java_array = vm.new_array(ArrayEntryType::Base(BaseType::Char), char_array.len());
    char_array
        .into_iter()
        .enumerate()
        .for_each(|(index, value)| java_array.set_element(index, value).unwrap());

    let string_object = vm.new_object(call_stack, "java/lang/String")?;
    string_object.set_field(0, Value::Object(java_array));
    string_object.set_field(1, Value::Int(0));
    string_object.set_field(6, Value::Int(0));
    Ok(string_object)
}

pub fn new_java_lang_class_object<'a>(
    vm: &mut Vm<'a>,
    call_stack: &mut CallStack<'a>,
    class_name: &str,
) -> Result<AbstractObject<'a>, MethodCallFailed<'a>> {
    let class_object = vm.new_object(call_stack, "java/lang/Class")?;
    let string_object = new_java_lang_string_object(vm, call_stack, class_name)?;
    class_object.set_field(5, Value::Object(string_object));

    Ok(class_object)
}
