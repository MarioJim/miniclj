use std::fmt::{self, Display};

use crate::{Callable, Scope, Value};

#[derive(Debug, Clone)]
struct NumberCast;

impl Display for NumberCast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "num")
    }
}

impl Callable for NumberCast {
    fn call(&self, _: &[Value], _: &Scope) -> Value {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct StringCast;

impl Display for StringCast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl Callable for StringCast {
    fn call(&self, _: &[Value], _: &Scope) -> Value {
        todo!()
    }
}
