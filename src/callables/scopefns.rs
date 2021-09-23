use std::fmt::{self, Display};

use crate::{Callable, Scope, Value};

#[derive(Debug, Clone)]
struct Def;

impl Display for Def {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "def")
    }
}

impl Callable for Def {
    fn call(&self, _: &[Value], _: &Scope) -> Value {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct Defn;

impl Display for Defn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "defn")
    }
}

impl Callable for Defn {
    fn call(&self, _: &[Value], _: &Scope) -> Value {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct Let;

impl Display for Let {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "let")
    }
}

impl Callable for Let {
    fn call(&self, _: &[Value], _: &Scope) -> Value {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct Loop;

impl Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "loop")
    }
}

impl Callable for Loop {
    fn call(&self, _: &[Value], _: &Scope) -> Value {
        todo!()
    }
}
