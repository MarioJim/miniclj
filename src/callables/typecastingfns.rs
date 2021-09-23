use crate::{
    callables::{Callable, ExecutionResult},
    Scope, Value,
};

#[derive(Debug, Clone)]
struct NumberCast;

impl Callable for NumberCast {
    fn name(&self) -> &'static str {
        "num"
    }

    fn call(&self, _: &[Value], _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(NumberCast);

#[derive(Debug, Clone)]
struct StringCast;

impl Callable for StringCast {
    fn name(&self) -> &'static str {
        "str"
    }
    fn call(&self, _: &[Value], _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(StringCast);
