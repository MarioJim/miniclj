use std::{borrow::Borrow, io::Write, rc::Rc};

pub mod callables;
pub mod callablestable;
pub mod constant;
pub mod error;
pub mod function;
pub mod instruction;
pub mod literal;
pub mod memaddress;
pub mod sexpr;
pub mod symboltable;

pub use callables::Callable;
pub use callablestable::CallablesTable;
pub use constant::Constant;
pub use error::{CompilationError, CompilationResult};
pub use instruction::Instruction;
pub use literal::Literal;
pub use memaddress::{DataType, MemAddress};
pub use sexpr::SExpr;
pub use symboltable::SymbolTable;

pub type InstructionPtr = usize;

#[derive(Debug, Default)]
pub struct State {
    instructions: Vec<Instruction>,
    symbol_table: Rc<SymbolTable>,
    callables_table: CallablesTable,
    constants: Vec<Constant>,
    temp_var_idx: usize,
}

impl State {
    pub fn compile(&mut self, expr: SExpr) -> CompilationResult {
        match expr {
            SExpr::Expr(symbol, exprs) => self
                .callables_table
                .get(&symbol)
                .ok_or(CompilationError::CallableNotDefined(symbol))?
                .compile(self, exprs),
            SExpr::List(_) => todo!(),
            SExpr::Vector(_) => todo!(),
            SExpr::Set(_) => todo!(),
            SExpr::Map(_) => todo!(),
            SExpr::Literal(lit) => {
                if let Literal::Symbol(symbol) = lit {
                    self.symbol_table
                        .get(&symbol)
                        .ok_or(CompilationError::SymbolNotDefined(symbol))
                } else {
                    let constant: Constant = lit.into();
                    let datatype = constant.data_type();
                    let idx = self.constants.len();
                    self.constants.push(constant);
                    Ok(MemAddress::new_constant(datatype, idx))
                }
            }
        }
    }

    pub fn push_scope(&mut self) {
        self.symbol_table = Rc::new(SymbolTable::new(Some(self.symbol_table.clone())));
    }

    pub fn pop_scope(&mut self) {
        self.symbol_table = match self.symbol_table.borrow() {
            SymbolTable::RootScope(_) => {
                panic!("Called compiler::Scope::pop_scope with the root scope")
            }
            SymbolTable::LocalScope(_, parent_table) => parent_table.clone(),
        };
    }

    pub fn write_to(&self, _file: impl Write) {
        todo!()
    }

    pub fn add_new_literals(&mut self, _literal: Literal) -> MemAddress {
        todo!()
    }

    pub fn instruction_ptr(&self) -> usize {
        self.instructions.len()
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction)
    }

    pub fn new_tmp_address(&mut self, datatype: DataType) -> MemAddress {
        let addr = MemAddress::new_temp(datatype, self.temp_var_idx);
        self.temp_var_idx += 1;
        addr
    }
}
