use std::{
    boxed::Box,
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

use num::Rational64;
use rand::random;

use crate::callables::Callable;

mod list;
mod map;
mod set;
mod sexpr;
mod vector;

pub use sexpr::SExpr;

#[derive(Debug, Clone)]
pub enum Value {
    SExpr(sexpr::SExpr),
    Fn(Box<dyn Callable>),

    List(list::List),
    Vector(vector::Vector),
    Set(set::Set),
    Map(map::Map),

    Identifier(String),
    String(String),
    // TODO: Maybe change to Ratio<isize>?
    Number(Rational64),
    Nil,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::SExpr(r), Value::SExpr(l)) => r == l,
            (Value::Fn(r), Value::Fn(l)) => r.name() == l.name(),
            (Value::List(r), Value::List(l)) => r == l,
            (Value::Vector(r), Value::Vector(l)) => r == l,
            (Value::Set(r), Value::Set(l)) => r == l,
            (Value::Map(r), Value::Map(l)) => r == l,
            (Value::Identifier(r), Value::Identifier(l)) => r == l,
            (Value::String(r), Value::String(l)) => r == l,
            (Value::Number(r), Value::Number(l)) => r == l,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }
}
impl Eq for Value {}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = match self {
            Self::SExpr(s) => s.to_string(),
            Self::Fn(f) => format!("#function[{}]", f.name()),
            Self::List(l) => l.to_string(),
            Self::Vector(v) => v.to_string(),
            Self::Set(s) => s.to_string(),
            Self::Map(m) => m.to_string(),
            Self::Identifier(i) => i.to_string(),
            Self::String(s) => format!("\"{}\"", s),
            Self::Number(n) => n.to_string(),
            Self::Nil => String::from("nil"),
        };
        write!(f, "{}", string)
    }
}

#[derive(Hash)]
struct NilHash;

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::SExpr(_) | Value::Fn(_) => {
                let x: u16 = random();
                x.hash(state)
            }
            Value::List(l) => l.hash(state),
            Value::Vector(v) => v.hash(state),
            Value::Set(s) => s.hash(state),
            Value::Map(m) => m.hash(state),
            Value::Identifier(i) => i.hash(state),
            Value::String(s) => s.hash(state),
            Value::Number(n) => n.hash(state),
            Value::Nil => NilHash.hash(state),
        }
    }
}

impl Value {
    pub fn type_str(&self) -> &'static str {
        match self {
            Value::SExpr(_) => "an s-expression",
            Value::Fn(_) => "a function",
            Value::List(_) => "a list",
            Value::Vector(_) => "a vector",
            Value::Set(_) => "a set",
            Value::Map(_) => "a map",
            Value::Identifier(_) => "a identifier",
            Value::String(_) => "a string",
            Value::Number(_) => "a number",
            Value::Nil => "nil",
        }
    }
}
