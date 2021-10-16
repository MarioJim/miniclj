use std::io::Write;

pub mod address;
pub mod callables;
pub mod instruction;
pub mod literal;
pub mod sexpr;
pub mod symboltable;

pub use address::Address;
pub use instruction::Instruction;
pub use literal::Literal;
pub use sexpr::SExpr;
pub use symboltable::SymbolTable;

#[derive(Debug, Default)]
pub struct State {
    literals: Vec<Literal>,
    instruction_ptr: usize,
    jumps_stack: Vec<usize>,
    temp_var_idx: usize,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn compile(&mut self, expr: SExpr, symbol_table: &mut SymbolTable) {
        match expr {
            SExpr::Expr(values) => {}
            SExpr::Lambda(_) => todo!(),
            SExpr::List(_) => todo!(),
            SExpr::Vector(_) => todo!(),
            SExpr::Set(_) => todo!(),
            SExpr::Map(_) => todo!(),
            SExpr::Literal(_) => todo!(),
        }
    }

    pub fn write_to(&self, file: impl Write) {
        todo!()
    }

    pub fn add_new_literals(&mut self, literal: Literal) -> Address {
        todo!()
    }
}
