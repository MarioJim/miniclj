use crate::{
    callables::{Callable, ExecutionResult},
    Scope, Value,
};

#[derive(Debug, Clone)]
pub struct LambdaFn;

impl Callable for LambdaFn {
    fn name(&self) -> &'static str {
        "fn"
    }

    fn call(&self, _: &[Value], _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(LambdaFn);

#[derive(Debug, Clone)]
pub struct AnonymousLambdaFn;

impl Callable for AnonymousLambdaFn {
    fn name(&self) -> &'static str {
        "an anonymous lambda"
    }

    fn call(&self, _: &[Value], _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(AnonymousLambdaFn);
