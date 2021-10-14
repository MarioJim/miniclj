use std::io::Write;

use super::SExpr;

#[derive(Debug, Default)]
pub struct State {
    program_name: String,
    instruction_ptr: usize,
    jumps_stack: Vec<usize>,
    temp_var_idx: usize,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn compile(&mut self, expr: SExpr) {
        todo!()
    }

    pub fn write_to(&self, file: impl Write) {
        todo!()
    }
}
