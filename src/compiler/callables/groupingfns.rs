use crate::compiler::{
    callables::{Callable, CompilationResult},
    SExpr, State,
};

#[derive(Debug, Clone)]
pub struct Do;

impl Callable for Do {
    fn name(&self) -> &'static str {
        "do"
    }

    fn compile(&self, _state: &mut State, _args: Vec<SExpr>) -> CompilationResult {
        todo!()
    }
}

display_for_callable!(Do);
