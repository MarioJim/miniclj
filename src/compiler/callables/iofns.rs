use std::rc::Rc;

use crate::compiler::{
    callables::{Callable, CompilationResult},
    SExpr, Scope, State,
};

#[derive(Debug, Clone)]
pub struct Print;

impl Callable for Print {
    fn name(&self) -> &'static str {
        "print"
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>, scope: &Rc<Scope>) -> CompilationResult {
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

    fn compile(&self, state: &mut State, _: Vec<SExpr>, _: &Rc<Scope>) -> CompilationResult {
        todo!()
    }
}

display_for_callable!(Read);
