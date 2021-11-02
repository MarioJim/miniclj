use std::collections::HashMap;

use crate::{
    compiler::CompilerState,
    constant::Constant,
    instruction::{Instruction, InstructionPtr},
    memaddress::MemAddress,
    vm::{RuntimeResult, Value},
};

#[derive(Debug)]
pub struct VMState {
    constants: HashMap<Constant, MemAddress>,
    temporals: Vec<Value>,
    instructions: Vec<Instruction>,
    instruction_ptr: InstructionPtr,
}

impl From<CompilerState> for VMState {
    fn from(compiler: CompilerState) -> Self {
        let (constants, instructions) = compiler.into_parts();

        VMState {
            constants,
            temporals: Vec::new(),
            instructions,
            instruction_ptr: 0,
        }
    }
}

impl TryFrom<String> for VMState {
    type Error = String;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        let _lines = string.lines();

        todo!()
    }
}

impl VMState {
    pub fn execute(&mut self) -> RuntimeResult {
        todo!()
    }

    pub fn instruction_ptr(&self) -> InstructionPtr {
        self.instruction_ptr
    }

    pub fn get(&self, address: &MemAddress) -> Value {
        panic!("Address {:?} not found", address);
    }

    pub fn store(&mut self, _address: MemAddress, _value: Value) {
        todo!()
    }
}
