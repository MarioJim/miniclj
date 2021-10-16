use std::rc::Rc;

use crate::compiler::{
    callables::{Callable, CompilationResult},
    SExpr, State, SymbolTable,
};

#[derive(Debug, Clone)]
pub struct First;

impl Callable for First {
    fn name(&self) -> &'static str {
        "first"
    }

    fn compile(
        &self,
        state: &mut State,
        args: Vec<SExpr>,
        scope: &Rc<SymbolTable>,
    ) -> CompilationResult {
        if args.len() != 1 {
            return self.arity_err("<collection>");
        }
        todo!()
    }
}

display_for_callable!(First);

#[derive(Debug, Clone)]
pub struct Rest;

impl Callable for Rest {
    fn name(&self) -> &'static str {
        "rest"
    }

    fn compile(
        &self,
        state: &mut State,
        args: Vec<SExpr>,
        scope: &Rc<SymbolTable>,
    ) -> CompilationResult {
        if args.len() != 1 {
            return self.arity_err("<collection>");
        }
        todo!()
    }
}

display_for_callable!(Rest);

#[derive(Debug, Clone)]
pub struct Cons;

impl Callable for Cons {
    fn name(&self) -> &'static str {
        "cons"
    }

    fn compile(
        &self,
        state: &mut State,
        args: Vec<SExpr>,
        scope: &Rc<SymbolTable>,
    ) -> CompilationResult {
        if args.len() != 2 {
            return self.arity_err("<value> <collection>");
        }
        todo!()
    }
}

display_for_callable!(Cons);

#[derive(Debug, Clone)]
pub struct Conj;

impl Callable for Conj {
    fn name(&self) -> &'static str {
        "conj"
    }

    fn compile(
        &self,
        state: &mut State,
        args: Vec<SExpr>,
        scope: &Rc<SymbolTable>,
    ) -> CompilationResult {
        if args.len() != 2 {
            return self.arity_err("<value> <collection>");
        }
        todo!()
    }
}

display_for_callable!(Conj);

#[derive(Debug, Clone)]
pub struct Get;

impl Callable for Get {
    fn name(&self) -> &'static str {
        "get"
    }

    fn compile(
        &self,
        state: &mut State,
        args: Vec<SExpr>,
        scope: &Rc<SymbolTable>,
    ) -> CompilationResult {
        if args.len() != 2 {
            return self.arity_err("<collection> <key>");
        }
        todo!()
    }
}

display_for_callable!(Get);

#[derive(Debug, Clone)]
pub struct Len;

impl Callable for Len {
    fn name(&self) -> &'static str {
        "len"
    }

    fn compile(
        &self,
        state: &mut State,
        args: Vec<SExpr>,
        scope: &Rc<SymbolTable>,
    ) -> CompilationResult {
        if args.len() != 1 {
            return self.arity_err("<collection>");
        }
        todo!()
    }
}

display_for_callable!(Len);

#[derive(Debug, Clone)]
pub struct IsEmpty;

impl Callable for IsEmpty {
    fn name(&self) -> &'static str {
        "empty?"
    }

    fn compile(
        &self,
        state: &mut State,
        args: Vec<SExpr>,
        scope: &Rc<SymbolTable>,
    ) -> CompilationResult {
        if args.len() != 1 {
            return self.arity_err("<collection>");
        }
        todo!()
    }
}

display_for_callable!(IsEmpty);
