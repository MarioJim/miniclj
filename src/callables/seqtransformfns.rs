use std::fmt::{self, Display};

use crate::{Callable, Scope, Value};

#[derive(Debug, Clone)]
struct Map;

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "map")
    }
}

impl Callable for Map {
    fn call(&self, _: &[Value], _: &Scope) -> Value {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct Filter;

impl Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "filter")
    }
}

impl Callable for Filter {
    fn call(&self, _: &[Value], _: &Scope) -> Value {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct Reduce;

impl Display for Reduce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "reduce")
    }
}

impl Callable for Reduce {
    fn call(&self, _: &[Value], _: &Scope) -> Value {
        todo!()
    }
}
