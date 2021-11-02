use crate::{
    callables::Callable,
    compiler::{CompilationError, CompilationResult, CompilerState},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ComparisonOp {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
}

impl Callable for ComparisonOp {
    fn name(&self) -> &'static str {
        match self {
            ComparisonOp::Eq => "==",
            ComparisonOp::Ne => "!=",
            ComparisonOp::Gt => ">",
            ComparisonOp::Lt => "<",
            ComparisonOp::Ge => ">=",
            ComparisonOp::Le => "<=",
        }
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 0 {
            Err(CompilationError::EmptyArgs(self.name()))
        } else {
            Ok(state.get_callable_addr(Box::new(*self)))
        }
    }
}

display_for_callable!(ComparisonOp);
