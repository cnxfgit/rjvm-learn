use log::{debug, warn};
use rjvm_reader::{
    instruction::{self, Instruction},
    line_number::LineNumber,
    program_counter::ProgramCounter,
    type_conversion::ToUsizeSafe,
};

use crate::{
    call_frame::InstructionCompleted::{ContinueMethodExecution, ReturnFromMethod}, call_stack::{self, CallStack}, class_and_method::ClassAndMethod, exception::{self, JavaException, MethodCallFailed}, object::Object, stack_trace_element::StackTraceElement, value::Value, value_stack::ValueStack, vm::Vm, vm_error::VmError
};

pub type MethodCallResult<'a> = Result<Option<Value<'a>>, MethodCallFailed<'a>>;

#[derive(Debug)]
pub struct CallFrame<'a> {
    class_and_method: ClassAndMethod<'a>,
    pc: ProgramCounter,
    locals: Vec<Value<'a>>,
    stack: ValueStack<'a>,
    code: &'a Vec<u8>,
}

enum InstructionCompleted<'a> {
    ReturnFromMethod(Option<Value<'a>>),
    ContinueMethodExecution,
}

impl<'a> CallFrame<'a> {
    pub fn to_stack_trace_element(&self) -> StackTraceElement<'a> {
        StackTraceElement {
            class_name: &self.class_and_method.class.name,
            method_name: &self.class_and_method.method.name,
            source_file: &self.class_and_method.class.source_file,
            line_number: self.get_line_number(),
        }
    }

    fn get_line_number(&self) -> Option<LineNumber> {
        if let Some(code) = self.class_and_method.method.code.as_ref() {
            if let Some(line_number_table) = &code.line_number_table {
                return Some(line_number_table.lookup_pc(self.pc));
            }
        }
        None
    }

    pub fn execute(
        &mut self,
        vm: &mut Vm<'a>,
        call_stack: &mut CallStack<'a>,
    ) -> MethodCallResult<'a> {
        self.debug_start_execution();

        loop {
            let executed_instruction_pc = self.pc;
            let (instruction, new_address) =
                Instruction::parse(self.code, executed_instruction_pc.0.into_usize_safe())
                    .map_err(|_| MethodCallFailed::InternalError(VmError::ValidationException))?;

            self.debug_print_status(&instruction);

            self.pc = ProgramCounter(new_address as u16);

            let instruction_result = self.execute_instruction(vm, call_stack, instruction);
            match instruction_result {
                Ok(ReturnFromMethod(return_value)) => return Ok(return_value),
                Ok(ContinueMethodExecution) => {}

                Err(MethodCallFailed::InternalError(err)) => {
                    return Err(MethodCallFailed::InternalError(err));
                }
                Err(MethodCallFailed::ExceptionThrown(exception)) => {
                    let exception_handler = self.find_exception_handler(
                        vm,
                        call_stack,
                        executed_instruction_pc,
                        &exception,
                    );

                    match exception_handler {
                        Err(err) => return Err(err),
                        Ok(None) => {
                            return Err(MethodCallFailed::ExceptionThrown(exception));
                        }
                        Ok(Some(catch_handler_pc)) => {
                            self.stack.push(Value::Object(exception.0))?;
                            self.pc = catch_handler_pc;
                        }
                    }
                }
            }
        }
    }

    fn execute_instruction(
        &mut self,
        vm: &mut Vm<'a>,
        call_stack: &mut CallStack<'a>,
        instruction: Instruction,
    ) -> Result<InstructionCompleted<'a>, MethodCallFailed<'a>> {
        match instruction {
            _ => {
                warn!("Unsupported instruction: {:?}", instruction);
                return Err(MethodCallFailed::InternalError(VmError::NotImplemented));
            }
        }
    }

    fn find_exception_handler(
        &self,
        vm: &mut Vm<'a>,
        call_stack: &mut CallStack<'a>,
        executed_instruction_pc: ProgramCounter,
        exception: &JavaException<'a>,
    ) -> Result<Option<ProgramCounter>, MethodCallFailed<'a>> {
        let exception_table = &self
            .class_and_method
            .method
            .code
            .as_ref()
            .unwrap()
            .exception_table;

        let catch_handlers = exception_table.lookup(executed_instruction_pc);

        for catch_handler in catch_handlers {
            match &catch_handler.catch_class {
                None => return Ok(Some((catch_handler.handler_pc))),
                Some(class_name) => {
                    let catch_class = vm.get_or_resolve_class(class_name)?;
                    let exception_class = vm.get_class_by_id(exception.0.class_id())?;
                    if exception_class.is_subclass_of(catch_class) {
                        return Ok(Some(catch_handler.handler_pc));
                    }
                }
            }
        }

        Ok(None)
    }

    fn debug_start_execution(&self) {
        debug!(
            "starting execution of method {}::{} - locals are {:?}",
            self.class_and_method.class.name, self.class_and_method.method.name, self.locals
        )
    }

    fn debug_print_status(&self, instruction: &Instruction) {
        debug!(
            "FRAME STATUS: executing {} signature {} pc: {}",
            self.to_stack_trace_element(),
            self.class_and_method.method.type_descriptor,
            self.pc
        );
        debug!("  stack:");
        for stack_entry in self.stack.iter() {
            debug!("  - {:?}", stack_entry);
        }
        debug!("  locals:");
        for local_variable in self.locals.iter() {
            debug!("  - {:?}", local_variable);
        }
        debug!("  next instruction: {:?}", instruction);
    }
}
