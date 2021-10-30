use crate::{
    callables::Callable,
    compiler::{CompilationResult, SExpr, State},
};

#[derive(Debug, Clone)]
pub struct Map;

impl Callable for Map {
    fn name(&self) -> &'static str {
        "map"
    }

    fn compile(&self, _state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 2 {
            return self.arity_err("<function> <collection>");
        }

        todo!()
    }
}

display_for_callable!(Map);

#[derive(Debug, Clone)]
pub struct Filter;

impl Callable for Filter {
    fn name(&self) -> &'static str {
        "filter"
    }

    fn compile(&self, _state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 2 {
            return self.arity_err("<function> <collection>");
        }

        todo!()
    }
}

display_for_callable!(Filter);

#[derive(Debug, Clone)]
pub struct Reduce;

impl Callable for Reduce {
    fn name(&self) -> &'static str {
        "reduce"
    }

    fn compile(&self, _state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 2 {
            return self.arity_err("<function> <collection>");
        }

        todo!()
    }
}

display_for_callable!(Reduce);
