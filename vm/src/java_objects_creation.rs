use rjvm_reader::{field_type::BaseType, line_number::LineNumber};

use crate::{
    abstract_object::{string_from_char_array, AbstractObject},
    array::Array,
    array_entry_type::ArrayEntryType,
    call_stack::CallStack,
    exceptions::MethodCallFailed,
    object::Object,
    stack_trace_element::StackTraceElement,
    value::Value,
    vm::Vm,
    vm_error::VmError,
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

pub fn extract_str_from_java_lang_string<'a>(
    vm: &mut Vm<'a>,
    object: &impl Object<'a>,
) -> Result<String, VmError> {
    let class = vm.get_class_by_id(object.class_id())?;
    if class.name == "java/lang/String" {
        if let Value::Object(array) = object.get_field(class, 0) {
            return string_from_char_array(array);
        }
    }
    Err(VmError::ValidationException)
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

pub fn new_java_lang_stack_trace_element_object<'a>(
    vm: &mut Vm<'a>,
    call_stack: &mut CallStack<'a>,
    stack_trace_element: &StackTraceElement<'a>,
) -> Result<AbstractObject<'a>, MethodCallFailed<'a>> {
    let class_name = Value::Object(new_java_lang_string_object(
        vm,
        call_stack,
        stack_trace_element.class_name,
    )?);
    let method_name = Value::Object(new_java_lang_string_object(
        vm,
        call_stack,
        &stack_trace_element.method_name,
    )?);
    let file_name = match stack_trace_element.source_file {
        Some(file_name) => Value::Object(new_java_lang_string_object(vm, call_stack, &file_name)?),
        _ => Value::Null,
    };
    let line_number = Value::Int(stack_trace_element.line_number.unwrap_or(LineNumber(0)).0 as i32);

    let stack_trace_element_java_object =
        vm.new_object(call_stack, "java/lang/StackTraceElement")?;
    stack_trace_element_java_object.set_field(0, class_name);
    stack_trace_element_java_object.set_field(1, method_name);
    stack_trace_element_java_object.set_field(2, file_name);
    stack_trace_element_java_object.set_field(3, line_number);

    Ok(stack_trace_element_java_object)
}
