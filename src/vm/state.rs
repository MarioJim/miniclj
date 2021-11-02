use std::{collections::HashMap, rc::Rc};

use crate::{
    constant::Constant,
    instruction::{Instruction, InstructionPtr},
    memaddress::{Lifetime, MemAddress},
    vm::{RuntimeError, RuntimeResult, Scope, Value},
};

#[derive(Debug)]
pub struct VMState {
    constants: HashMap<MemAddress, Constant>,
    instructions: Vec<Instruction>,
    scope: Rc<Scope>,
    instruction_ptr: InstructionPtr,
}

impl VMState {
    pub fn new(
        constants: HashMap<MemAddress, Constant>,
        instructions: Vec<Instruction>,
    ) -> VMState {
        VMState {
            constants,
            instructions,
            scope: Default::default(),
            instruction_ptr: 0,
        }
    }

    pub fn execute(&mut self) -> RuntimeResult {
        while let Some(instruction) = self.instructions.get(self.instruction_ptr) {
            match instruction {
                Instruction::Call {
                    callable: callable_addr,
                    args: arg_addrs,
                    result_addr,
                } => {
                    let callable = self.get(callable_addr);
                    let args = arg_addrs.iter().map(|addr| self.get(addr)).collect();
                    match callable {
                        Value::Callable(builtin_callable) => {
                            let result = builtin_callable.execute(args)?;
                            self.store(*result_addr, result);
                            Ok(())
                        }
                        Value::Lambda(new_instruction_ptr, arity) => {
                            if args.len() != arity {
                                return Err(RuntimeError::WrongArity(arity, args.len()));
                            }

                            self.scope = Rc::new(Scope::new(
                                *result_addr,
                                self.instruction_ptr,
                                self.scope.clone(),
                            ));
                            for (idx, arg) in args.into_iter().enumerate() {
                                self.store(MemAddress::new_local_var(idx), arg);
                            }
                            self.instruction_ptr = new_instruction_ptr;

                            Ok(())
                        }
                        _ => Err(RuntimeError::NotACallable(callable.type_str())),
                    }
                }
                Instruction::Return(return_addr) => {
                    let return_value = self.scope.get(return_addr);
                    let (top_return_address, top_instruction_ptr, top_scope) =
                        self.scope.top_state();

                    self.scope = top_scope;
                    self.instruction_ptr = top_instruction_ptr;
                    self.store(top_return_address, return_value);

                    Ok(())
                }
                Instruction::Assignment { src, dst } => {
                    self.store(*dst, self.get(src));
                    Ok(())
                }
                Instruction::Jump(new_instr_ptr) => {
                    self.instruction_ptr = *new_instr_ptr;
                    Ok(())
                }
                Instruction::JumpOnTrue(addr, new_instr_ptr) => {
                    let condition = self.get(addr).as_bool().map_err(|type_str| {
                        RuntimeError::WrongDataType("jmpT", "a 0/1 number", type_str)
                    })?;
                    if condition {
                        self.instruction_ptr = *new_instr_ptr;
                    }
                    Ok(())
                }
                Instruction::JumpOnFalse(addr, new_instr_ptr) => {
                    let condition = self.get(addr).as_bool().map_err(|type_str| {
                        RuntimeError::WrongDataType("jmpF", "a 0/1 number", type_str)
                    })?;
                    if !condition {
                        self.instruction_ptr = *new_instr_ptr;
                    }
                    Ok(())
                }
            }?;
        }

        Ok(())
    }

    pub fn get(&self, address: &MemAddress) -> Value {
        match address.lifetime() {
            Lifetime::Constant => self.constants.get(address).unwrap().clone().into(),
            _ => self.scope.get(address),
        }
    }

    pub fn store(&self, address: MemAddress, value: Value) {
        self.scope.insert(address, value);
    }
}
