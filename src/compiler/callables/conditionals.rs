use std::rc::Rc;

use num::Zero;

use crate::compiler::{
    callables::{Callable, CompilationResult},
    Literal, SExpr, Scope, State,
};

#[derive(Debug, Clone)]
pub struct IsTrue;

impl IsTrue {
    pub fn inner_call(&self, val: &Literal) -> bool {
        match val {
            Literal::Symbol(_) => {
                unreachable!("IsTrue::inner_call called with a symbol")
            }
            Literal::Number(n) => !n.is_zero(),
            Literal::Nil => false,
            _ => true,
        }
    }
}

impl Callable for IsTrue {
    fn name(&self) -> &'static str {
        "true?"
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>, scope: &Rc<Scope>) -> CompilationResult {
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

    fn compile(&self, state: &mut State, args: Vec<SExpr>, scope: &Rc<Scope>) -> CompilationResult {
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

    fn compile(&self, state: &mut State, args: Vec<SExpr>, scope: &Rc<Scope>) -> CompilationResult {
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

    fn compile(&self, state: &mut State, args: Vec<SExpr>, scope: &Rc<Scope>) -> CompilationResult {
        todo!()
    }
}

display_for_callable!(Or);
