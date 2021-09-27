use crate::{
    callables::{Callable, ExecutionResult},
    Scope, Value,
};

#[derive(Debug, Clone)]
pub struct Do;

impl Callable for Do {
    fn name(&self) -> &'static str {
        "do"
    }

    fn call(&self, args: &[Value], scope: &Scope) -> ExecutionResult {
        let mut result = Value::Nil;
        for arg in args {
            result = arg.eval(scope)?;
        }
        Ok(result)
    }
}

display_for_callable!(Do);
