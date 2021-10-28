use crate::compiler::{
    callables::{Callable, CompilationResult},
    SExpr, State,
};

#[derive(Debug, Clone)]
pub struct IsTrue;

impl Callable for IsTrue {
    fn name(&self) -> &'static str {
        "true?"
    }

    fn compile(&self, _state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 1 {
            return self.arity_err("<value>");
        }
        todo!()
    }
}

display_for_callable!(IsTrue);

#[derive(Debug, Clone)]
pub struct If;

impl Callable for If {
    fn name(&self) -> &'static str {
        "if"
    }

    fn compile(&self, _state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 3 {
            return self.arity_err("<condition> <true expression> <false expression>");
        }
        todo!()
    }
}

display_for_callable!(If);

#[derive(Debug, Clone)]
pub struct And;

impl Callable for And {
    fn name(&self) -> &'static str {
        "and"
    }

    fn compile(&self, _state: &mut State, _args: Vec<SExpr>) -> CompilationResult {
        todo!()
    }
}

display_for_callable!(And);

#[derive(Debug, Clone)]
pub struct Or;

impl Callable for Or {
    fn name(&self) -> &'static str {
        "or"
    }

    fn compile(&self, _state: &mut State, _args: Vec<SExpr>) -> CompilationResult {
        todo!()
    }
}

display_for_callable!(Or);
