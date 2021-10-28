use crate::compiler::{
    callables::{Callable, CompilationResult},
    SExpr, State,
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
            ComparisonOp::Eq => "=",
            ComparisonOp::Ne => "!=",
            ComparisonOp::Gt => ">",
            ComparisonOp::Lt => "<",
            ComparisonOp::Ge => ">=",
            ComparisonOp::Le => "<=",
        }
    }

    fn compile(&self, _state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.is_empty() {
            return self.arity_err("<...args>");
        }
        todo!()
    }
}

display_for_callable!(ComparisonOp);
