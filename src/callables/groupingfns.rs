use std::fmt::{self, Display};

use crate::{Callable, Scope, Value};

#[derive(Debug, Clone)]
struct Do;

impl Display for Do {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "do")
    }
}

impl Callable for Do {
    fn call(&self, _: &[Value], _: &Scope) -> Value {
        todo!()
    }
}
