use crate::{
    callables::{Callable, ExecutionResult},
    Scope, Value,
};

#[derive(Debug, Clone)]
struct Do;

impl Callable for Do {
    fn name(&self) -> &'static str {
        "do"
    }

    fn call(&self, _: &[Value], _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(Do);
