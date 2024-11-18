use log::warn;
use rjvm_reader::instruction::{self, Instruction};

use crate::{
    call_stack::CallStack, exception::{self, MethodCallFailed}, value::Value, vm::Vm, vm_error::VmError,
};

pub type MethodCallResult<'a> = Result<Option<Value<'a>>, MethodCallFailed<'a>>;

#[derive(Debug)]
pub struct CallFrame<'a> {
    locals: Vec<Value<'a>>,
}

enum InstructionCompleted<'a> {
    ReturnFromMethod(Option<Value<'a>>),
    ContinueMethodExecution,
}

impl<'a> CallFrame<'a> {
    pub fn execute(
        &mut self,
        vm: &mut Vm<'a>,
        call_stack: &mut CallStack<'a>,
    ) -> MethodCallResult<'a> {
        self.debug_start_execution();

        loop {
            let executed_instruction_pc = self.pc;
            let (instruction, new_address) = 
                Instruction::parse(
                    self.code,
                    executed_instruction_pc.0.into_usize_safe()
                ).map_err(|_| MethodCallFailed::InternalError(
                    VmError::ValidationException)
                )?;

            self.debug_print_status(&instruction);
            
            self.pc = ProgramCounter(new_address as u16)

            let instruction_result = 
                self.execute_instruction(vm, call_stack, instruction);
            match instruction_result {
                Ok(ReturnFromMethod(return_value)) => return Ok(return_value),
                Ok(ContinueMethodExecution) => {},

                Err(MethodCallFailed::InternalError(err)) => {
                    return Err(MethodCallFailed::InternalError(err));
                },
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
                        },
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
}
