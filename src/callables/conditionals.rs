use std::fmt::{self, Display};

use crate::{Callable, Scope, Value};

#[derive(Debug, Clone)]
struct If;

impl Display for If {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "if")
    }
}

impl Callable for If {
    fn call(&self, _: &[Value], _: &Scope) -> Value {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct And;

impl Display for And {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "and")
    }
}

impl Callable for And {
    fn call(&self, _: &[Value], _: &Scope) -> Value {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct Or;

impl Display for Or {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "or")
    }
}

impl Callable for Or {
    fn call(&self, _: &[Value], _: &Scope) -> Value {
        todo!()
    }
}
