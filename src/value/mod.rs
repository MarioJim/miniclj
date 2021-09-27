use std::{
    boxed::Box,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

use num::Rational64;
use rand::random;

use crate::{
    callables::{Callable, ExecutionResult, RuntimeError},
    Scope,
};

pub mod list;
pub mod map;
pub mod set;
pub mod sexpr;
pub mod vector;

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

    pub fn eval(&self, scope: &Scope) -> ExecutionResult {
        match self {
            Value::SExpr(_) => todo!(),
            Value::Identifier(id) => match scope.get(id) {
                Some(val) => val.eval(scope),
                None => Err(RuntimeError::NotDefined(id.clone())),
            },
            _ => Ok(self.clone()),
        }
    }
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
            Value::SExpr(s) => s.to_string(),
            Value::Fn(f) => format!("#function[{}]", f.name()),
            Value::List(l) => l.to_string(),
            Value::Vector(v) => v.to_string(),
            Value::Set(s) => s.to_string(),
            Value::Map(m) => m.to_string(),
            Value::Identifier(i) => i.to_string(),
            Value::String(s) => format!("\"{}\"", s),
            Value::Number(n) => n.to_string(),
            Value::Nil => String::from("nil"),
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

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Value::Number(Rational64::from_integer(n))
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::from(if b { 1 } else { 0 })
    }
}

pub enum ValueIterator {
    List(list::ListIter),
    Vector(vector::VectorIter),
    Set(set::SetIter),
    Map(map::MapIter),
}

impl TryFrom<Value> for ValueIterator {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::List(l) => Ok(ValueIterator::List(l.into_iter())),
            Value::Vector(v) => Ok(ValueIterator::Vector(v.into_iter())),
            Value::Set(s) => Ok(ValueIterator::Set(s.into_iter())),
            Value::Map(m) => Ok(ValueIterator::Map(m.into_iter())),
            _ => Err(()),
        }
    }
}

impl Iterator for ValueIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ValueIterator::List(it) => it.next(),
            ValueIterator::Vector(it) => it.next(),
            ValueIterator::Set(it) => it.next(),
            ValueIterator::Map(it) => it
                .next()
                .map(|(k, v)| Value::Vector(vector::Vector::from(vec![k, v]))),
        }
    }
}
