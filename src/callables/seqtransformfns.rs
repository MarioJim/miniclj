use crate::{
    callables::{Callable, ExecutionResult},
    Scope, Value,
};

#[derive(Debug, Clone)]
struct Map;

impl Callable for Map {
    fn name(&self) -> &'static str {
        "map"
    }

    fn call(&self, _: &[Value], _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(Map);

#[derive(Debug, Clone)]
struct Filter;

impl Callable for Filter {
    fn name(&self) -> &'static str {
        "filter"
    }

    fn call(&self, _: &[Value], _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(Filter);

#[derive(Debug, Clone)]
struct Reduce;

impl Callable for Reduce {
    fn name(&self) -> &'static str {
        "reduce"
    }

    fn call(&self, _: &[Value], _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(Reduce);
