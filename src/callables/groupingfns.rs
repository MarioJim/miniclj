use std::fmt::{self, Display};

use crate::{callables::Callable, value::Value};

#[derive(Debug, Clone)]
struct Do;

impl Display for Do {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "do")
    }
}

impl Callable for Do {
    fn call(&self, _: &[Value]) -> Value {
        todo!()
    }
}
