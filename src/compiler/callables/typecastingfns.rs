use std::rc::Rc;

use crate::compiler::{
    callables::{Callable, CompilationResult},
    SExpr, State, SymbolTable,
};

#[derive(Debug, Clone)]
pub struct NumberCast;

impl Callable for NumberCast {
    fn name(&self) -> &'static str {
        "num"
    }

    fn compile(
        &self,
        state: &mut State,
        args: Vec<SExpr>,
        scope: &Rc<SymbolTable>,
    ) -> CompilationResult {
        if args.len() != 1 {
            return self.arity_err("<string>");
        }
        todo!()
    }
}

display_for_callable!(NumberCast);

#[derive(Debug, Clone)]
pub struct StringCast;

impl Callable for StringCast {
    fn name(&self) -> &'static str {
        "str"
    }

    fn compile(
        &self,
        state: &mut State,
        args: Vec<SExpr>,
        scope: &Rc<SymbolTable>,
    ) -> CompilationResult {
        todo!()
    }
}

display_for_callable!(StringCast);

#[derive(Debug, Clone)]
pub struct Ord;

impl Callable for Ord {
    fn name(&self) -> &'static str {
        "ord"
    }

    fn compile(
        &self,
        state: &mut State,
        args: Vec<SExpr>,
        scope: &Rc<SymbolTable>,
    ) -> CompilationResult {
        if args.len() != 1 {
            return self.arity_err("<string>");
        }
        todo!()
    }
}

display_for_callable!(Ord);

#[derive(Debug, Clone)]
pub struct Chr;

impl Callable for Chr {
    fn name(&self) -> &'static str {
        "chr"
    }

    fn compile(
        &self,
        state: &mut State,
        args: Vec<SExpr>,
        scope: &Rc<SymbolTable>,
    ) -> CompilationResult {
        if args.len() != 1 {
            return self.arity_err("<number>");
        }
        todo!()
    }
}

display_for_callable!(Chr);
