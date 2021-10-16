use std::rc::Rc;

use crate::compiler::{
    callables::{Callable, CompilationError, CompilationResult},
    Literal, SExpr, State, SymbolTable,
};

#[derive(Debug, Clone)]
pub struct Def;

impl Callable for Def {
    fn name(&self) -> &'static str {
        "def"
    }

    fn compile(
        &self,
        state: &mut State,
        args: Vec<SExpr>,
        scope: &Rc<SymbolTable>,
    ) -> CompilationResult {
        if args.len() != 2 {
            return self.arity_err("<symbol> <value>");
        }
        todo!()
    }
}

display_for_callable!(Def);

#[derive(Debug, Clone)]
pub struct Defn;

impl Callable for Defn {
    fn name(&self) -> &'static str {
        "defn"
    }

    fn compile(
        &self,
        state: &mut State,
        args: Vec<SExpr>,
        scope: &Rc<SymbolTable>,
    ) -> CompilationResult {
        if args.len() != 3 {
            return self.arity_err("<symbol> <arguments vector> <expression>");
        }

        let mut args_iter = args.into_iter();
        let maybe_symbol = args_iter.next().unwrap();
        let symbol = if let SExpr::Literal(Literal::Symbol(sym)) = maybe_symbol {
            sym
        } else {
            return Err(CompilationError::WrongArgument(
                self.name(),
                "a symbol",
                maybe_symbol.type_str(),
            ));
        };

        todo!()
    }
}

display_for_callable!(Defn);

#[derive(Debug, Clone)]
pub struct Let;

impl Callable for Let {
    fn name(&self) -> &'static str {
        "let"
    }

    fn compile(
        &self,
        state: &mut State,
        args: Vec<SExpr>,
        scope: &Rc<SymbolTable>,
    ) -> CompilationResult {
        if args.len() != 2 {
            return self.arity_err("<vector of bindings> <body>");
        }

        let first_arg_error = Err(CompilationError::Error(format!(
            "First argument of {} must be a vector with symbol - value pairs",
            self.name()
        )));

        let mut args_iter = args.into_iter();
        let bindings_vector = if let SExpr::Vector(v) = args_iter.next().unwrap() {
            v
        } else {
            return first_arg_error;
        };

        todo!()
    }
}

display_for_callable!(Let);

#[derive(Debug, Clone)]
pub struct Loop;

impl Callable for Loop {
    fn name(&self) -> &'static str {
        "loop"
    }

    fn compile(&self, _: &mut State, _: Vec<SExpr>, _: &Rc<SymbolTable>) -> CompilationResult {
        todo!()
    }
}

display_for_callable!(Loop);

#[derive(Debug, Clone)]
pub struct Recur;

impl Callable for Recur {
    fn name(&self) -> &'static str {
        "recur"
    }

    fn compile(&self, _: &mut State, _: Vec<SExpr>, _: &Rc<SymbolTable>) -> CompilationResult {
        todo!()
    }
}

display_for_callable!(Recur);
