use crate::{
    callables::{Callable, ExecutionResult},
    value::SExpr,
    Scope, Value,
};

#[derive(Debug, Clone)]
pub struct Do;

impl Callable for Do {
    fn name(&self) -> &'static str {
        "do"
    }

    fn call(&self, args: Vec<SExpr>, scope: &Scope) -> ExecutionResult {
        let mut result = Value::Nil;
        for arg in args {
            result = arg.eval(scope)?;
        }
        Ok(result)
    }
}

display_for_callable!(Do);
