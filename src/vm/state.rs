use std::collections::HashMap;

use crate::{
    callables::CallableResult,
    constant::Constant,
    instruction::{Instruction, InstructionPtr},
    memaddress::{Lifetime, MemAddress},
    vm::{RuntimeError, RuntimeResult, Scope, Value},
};

#[derive(Debug)]
pub struct VMState {
    constants: HashMap<MemAddress, Constant>,
    instructions: Vec<Instruction>,
    global_scope: Scope,
}

impl VMState {
    pub fn new(
        constants: HashMap<MemAddress, Constant>,
        instructions: Vec<Instruction>,
    ) -> VMState {
        VMState {
            constants,
            instructions,
            global_scope: Scope::default(),
        }
    }

    pub fn execute(&self) -> RuntimeResult {
        match self.inner_execute(0, &self.global_scope)? {
            Some(addr) => Err(RuntimeError::CompilerError(format!(
                "Trying to return address {} from the root scope",
                addr
            ))),
            None => Ok(()),
        }
    }

    pub fn execute_lambda(
        &self,
        new_instruction_ptr: InstructionPtr,
        arity: usize,
        args: Vec<Value>,
    ) -> CallableResult {
        if args.len() != arity {
            return Err(RuntimeError::WrongArity(arity, args.len()));
        }

        let local_scope = Scope::default();
        for (idx, arg) in args.into_iter().enumerate() {
            self.store(&local_scope, MemAddress::new_local_var(idx), arg)?;
        }

        match self.inner_execute(new_instruction_ptr, &local_scope)? {
            Some(return_address) => self.get(&local_scope, &return_address),
            None => Err(RuntimeError::CompilerError(format!(
                "User defined callable at {} never returned",
                new_instruction_ptr
            ))),
        }
    }

    fn inner_execute(
        &self,
        starting_instruction_ptr: usize,
        current_scope: &Scope,
    ) -> Result<Option<MemAddress>, RuntimeError> {
        let mut instruction_ptr = starting_instruction_ptr;

        while let Some(instruction) = self.instructions.get(instruction_ptr) {
            match instruction {
                Instruction::Call {
                    callable: callable_addr,
                    args: arg_addrs,
                    result_addr,
                } => {
                    let callable = self.get(current_scope, callable_addr)?;
                    let args = arg_addrs
                        .iter()
                        .map(|addr| self.get(current_scope, addr))
                        .collect::<Result<Vec<Value>, RuntimeError>>()?;
                    match callable {
                        Value::Callable(builtin_callable) => {
                            let result = builtin_callable.execute(args)?;
                            self.store(current_scope, *result_addr, result)?;
                            instruction_ptr += 1;
                            Ok(())
                        }
                        Value::Lambda(new_instruction_ptr, arity) => {
                            let result = self.execute_lambda(new_instruction_ptr, arity, args)?;
                            self.store(current_scope, *result_addr, result)?;
                            instruction_ptr += 1;
                            Ok(())
                        }
                        _ => Err(RuntimeError::NotACallable(callable.type_str())),
                    }
                }
                Instruction::Return(return_addr) => return Ok(Some(*return_addr)),
                Instruction::Assignment { src, dst } => {
                    let value = self.get(current_scope, src)?;
                    self.store(current_scope, *dst, value)?;
                    instruction_ptr += 1;
                    Ok(())
                }
                Instruction::Jump(new_instr_ptr) => {
                    instruction_ptr = *new_instr_ptr;
                    Ok(())
                }
                Instruction::JumpOnTrue(addr, new_instr_ptr) => {
                    let condition =
                        self.get(current_scope, addr)?
                            .as_bool()
                            .map_err(|type_str| {
                                RuntimeError::CompilerError(format!(
                                    "jmpT instruction at {} received {}, expected a 0/1 number",
                                    instruction_ptr, type_str
                                ))
                            })?;
                    if condition {
                        instruction_ptr = *new_instr_ptr;
                    }
                    Ok(())
                }
                Instruction::JumpOnFalse(addr, new_instr_ptr) => {
                    let condition =
                        self.get(current_scope, addr)?
                            .as_bool()
                            .map_err(|type_str| {
                                RuntimeError::CompilerError(format!(
                                    "jmpF instruction at {} received {}, expected a 0/1 number",
                                    instruction_ptr, type_str
                                ))
                            })?;
                    if !condition {
                        instruction_ptr = *new_instr_ptr;
                    }
                    Ok(())
                }
            }?;
        }

        Ok(None)
    }

    pub fn get(&self, current_scope: &Scope, address: &MemAddress) -> Result<Value, RuntimeError> {
        match address.lifetime() {
            Lifetime::Constant => self
                .constants
                .get(address)
                .ok_or_else(|| {
                    RuntimeError::CompilerError(
                        "Memory address not found in constants table".to_string(),
                    )
                })
                .map(|constant| constant.clone().into()),
            Lifetime::GlobalVar => self.global_scope.get_var(address.idx()),
            Lifetime::LocalVar => current_scope.get_var(address.idx()),
            Lifetime::Temporal => current_scope.get_temp(address.idx()),
        }
    }

    pub fn store(&self, current_scope: &Scope, address: MemAddress, value: Value) -> RuntimeResult {
        let index = address.idx();
        match address.lifetime() {
            Lifetime::Constant => Err(RuntimeError::CompilerError(
                "Memory address not found in constants table".to_string(),
            )),
            Lifetime::GlobalVar => {
                self.global_scope.store_var(index, value);
                Ok(())
            }
            Lifetime::LocalVar => {
                current_scope.store_var(index, value);
                Ok(())
            }
            Lifetime::Temporal => {
                current_scope.store_temp(index, value);
                Ok(())
            }
        }
    }
}
