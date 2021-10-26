use std::{borrow::Borrow, collections::VecDeque, io::Write, rc::Rc};

pub mod callables;
pub mod constant;
pub mod error;
pub mod function;
pub mod instruction;
pub mod literal;
pub mod memaddress;
pub mod sexpr;
pub mod symboltable;

pub use callables::Callable;
pub use constant::Constant;
pub use error::{CompilationError, CompilationResult};
pub use instruction::Instruction;
pub use literal::Literal;
pub use memaddress::MemAddress;
pub use sexpr::SExpr;
pub use symboltable::SymbolTable;

use self::callables::FactorOp;

pub type InstructionPtr = usize;

#[derive(Debug, Default)]
pub struct State {
    instructions: Vec<Instruction>,
    symbol_table: Rc<SymbolTable>,
    constants: Vec<Constant>,
    callables: Vec<Box<dyn Callable>>,
}

impl State {
    pub fn new() -> State {
        let mut state = State::default();
        for c in [FactorOp::Add, FactorOp::Sub, FactorOp::Mul, FactorOp::Div] {
            let idx = state.callables.len();
            state
                .symbol_table
                .insert(c.name().to_string(), MemAddress::new_builtin_callable(idx));
            state.callables.push(Box::new(c));
        }
        state
    }

    pub fn compile(&mut self, expr: SExpr) -> CompilationResult {
        match expr {
            SExpr::Expr(values) => {
                let mut params_queue = values.into_iter().map(|v| *v).collect::<VecDeque<_>>();
                let callable_addr = match params_queue.pop_front() {
                    Some(expr) => match expr {
                        SExpr::Expr(_) => todo!(),
                        SExpr::Lambda(_) => todo!(),
                        SExpr::List(_) => todo!(),
                        SExpr::Vector(_) => todo!(),
                        SExpr::Set(_) => todo!(),
                        SExpr::Map(_) => todo!(),
                        SExpr::Literal(_) => todo!(),
                    },
                    None => todo!(), // Call list
                };

                todo!()
            }
            SExpr::Lambda(_) => todo!(),
            SExpr::List(_) => todo!(),
            SExpr::Vector(_) => todo!(),
            SExpr::Set(_) => todo!(),
            SExpr::Map(_) => todo!(),
            SExpr::Literal(lit) => {
                if let Literal::Symbol(symbol) = lit {
                    self.symbol_table
                        .get(&symbol)
                        .ok_or(CompilationError::NotDefined(symbol))
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

    pub fn write_to(&self, file: impl Write) {
        todo!()
    }

    pub fn add_new_literals(&mut self, literal: Literal) -> MemAddress {
        todo!()
    }

    pub fn instruction_ptr(&self) -> usize {
        self.instructions.len()
    }
}
