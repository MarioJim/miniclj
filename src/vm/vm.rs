use crate::compiler::{self, Instruction};

#[derive(Debug)]
pub struct State {
    instructions: Vec<Instruction>,
}

impl State {
    pub fn from_compiler_state(compiler: compiler::State) -> Self {
        todo!()
    }

    pub fn try_from_string(string: String) -> Result<Self, String> {
        todo!()
    }

    pub fn execute(&mut self) {
        todo!()
    }
}
