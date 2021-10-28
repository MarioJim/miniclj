use crate::compiler::{
    callables::{Callable, CompilationResult},
    SExpr, State,
};

#[derive(Debug, Clone)]
pub struct Print;

impl Callable for Print {
    fn name(&self) -> &'static str {
        "print"
    }

    fn compile(&self, _state: &mut State, _args: Vec<SExpr>) -> CompilationResult {
        todo!()
    }
}

display_for_callable!(Print);

#[derive(Debug, Clone)]
pub struct Read;

impl Callable for Read {
    fn name(&self) -> &'static str {
        "read"
    }

    fn compile(&self, _state: &mut State, _: Vec<SExpr>) -> CompilationResult {
        todo!()
    }
}

display_for_callable!(Read);
