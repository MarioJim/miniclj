use std::fmt::{self, Display};

use crate::{callables::Callable, value::Value};

#[derive(Debug, Clone)]
struct If;

impl Display for If {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "if")
    }
}

impl Callable for If {
    fn call(&self, _: &[Value]) -> Value {
        todo!()
    }
}
