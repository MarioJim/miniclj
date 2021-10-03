use crate::{
    callables::{Callable, ExecutionResult},
    SExpr, Scope,
};

#[derive(Debug, Clone)]
pub struct LambdaFn;

impl Callable for LambdaFn {
    fn name(&self) -> &'static str {
        "fn"
    }

    fn call(&self, _: Vec<SExpr>, _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(LambdaFn);

#[derive(Debug, Clone)]
pub struct AnonymousLambdaFn;

impl Callable for AnonymousLambdaFn {
    fn name(&self) -> &'static str {
        "*lambda"
    }

    fn call(&self, _: Vec<SExpr>, _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(AnonymousLambdaFn);
