use std::rc::Rc;

use crate::compiler::{
    callables::{Callable, CompilationResult},
    SExpr, Scope, State,
};

#[derive(Debug, Clone)]
pub struct Do;

impl Callable for Do {
    fn name(&self) -> &'static str {
        "do"
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>, scope: &Rc<Scope>) -> CompilationResult {
        todo!()
    }
}

display_for_callable!(Do);
