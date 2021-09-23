use std::fmt::{self, Display};

use crate::{callables::Callable, value::Value};

#[derive(Debug, Clone)]
struct LambdaFn;

impl Display for LambdaFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn")
    }
}

impl Callable for LambdaFn {
    fn call(&self, _: &[Value]) -> Value {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct AnonymousLambdaFn;

impl Display for AnonymousLambdaFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl Callable for AnonymousLambdaFn {
    fn call(&self, _: &[Value]) -> Value {
        todo!()
    }
}
