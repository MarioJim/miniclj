use std::{
    boxed::Box,
    fmt::{self, Display, Formatter},
};

use num::BigRational;

mod callable;
mod list;

pub enum Value {
    DefMacro,
    Fn(Box<dyn callable::Callable>),

    List(list::List),
    Vector(Vec<Value>),
    Set(Vec<Value>),

    Identifier(String),
    String(String),
    Number(BigRational),
    Nil,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::DefMacro, Self::DefMacro) => true,
            (Self::Fn { .. }, Self::Fn { .. }) => false,
            (Self::List(l1), Self::List(l2)) => l1 == l2,
            (Self::Vector(v1), Self::Vector(v2)) => v1 == v2,
            (Self::Set(s1), Self::Set(s2)) => s1 == s2,
            (Self::Identifier(i1), Self::Identifier(i2)) => i1 == i2,
            (Self::String(s1), Self::String(s2)) => s1 == s2,
            (Self::Number(n1), Self::Number(n2)) => n1 == n2,
            (Self::Nil, Self::Nil) => true,
            _ => false,
        }
    }
}
impl Eq for Value {}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = match self {
            Self::DefMacro => String::from("#def"),
            Self::Fn(_) => String::from("#function"),
            Self::List(l) => l.to_string(),
            Self::Vector(_) => todo!(),
            Self::Set(_) => todo!(),
            Self::Identifier(i) => i.to_string(),
            Self::String(s) => format!("\"{}\"", s),
            Self::Number(n) => n.to_string(),
            Self::Nil => String::from("nil"),
        };
        write!(f, "{}", string)
    }
}

impl Value {}
