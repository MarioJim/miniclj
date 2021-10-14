use std::rc::Rc;

use crate::compiler::{
    callables::{Callable, CompilationResult},
    SExpr, Scope, State,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FactorOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl Callable for FactorOp {
    fn name(&self) -> &'static str {
        match self {
            FactorOp::Add => "+",
            FactorOp::Sub => "-",
            FactorOp::Mul => "*",
            FactorOp::Div => "/",
        }
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>, scope: &Rc<Scope>) -> CompilationResult {
        todo!()
    }
}

display_for_callable!(FactorOp);
