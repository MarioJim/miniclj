use std::rc::Rc;

use crate::compiler::{
    callables::{Callable, CompilationResult},
    SExpr, Scope, State,
};

#[derive(Debug, Clone)]
pub struct LambdaFn {
    symbols: Vec<String>,
    body: SExpr,
}

impl LambdaFn {
    pub fn new(symbols: Vec<String>, body: SExpr) -> LambdaFn {
        LambdaFn { symbols, body }
    }

    pub fn new_from_literal(body: SExpr) -> LambdaFn {
        LambdaFn::new(vec![String::from("%")], body)
    }
}

impl Callable for LambdaFn {
    fn name(&self) -> &'static str {
        "*fn"
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>, scope: &Rc<Scope>) -> CompilationResult {
        todo!()
    }

    fn is_user_defined(&self) -> bool {
        true
    }
}

display_for_callable!(LambdaFn);
#[derive(Debug, Clone)]
pub struct AnonymousFn;

impl Callable for AnonymousFn {
    fn name(&self) -> &'static str {
        "fn"
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>, _: &Rc<Scope>) -> CompilationResult {
        if args.len() != 2 {
            return self.arity_err("<arguments vector> <expression>");
        }
        todo!()
    }
}

display_for_callable!(AnonymousFn);
