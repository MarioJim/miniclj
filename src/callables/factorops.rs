use crate::{
    callables::Callable,
    compiler::{CompilationError, CompilationResult, CompilerState},
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

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        match (self, num_args) {
            (FactorOp::Sub, 0) | (FactorOp::Div, 0) => {
                Err(CompilationError::EmptyArgs(self.name()))
            }
            _ => Ok(state.get_callable_addr(Box::new(*self))),
        }
    }
}

display_for_callable!(FactorOp);
