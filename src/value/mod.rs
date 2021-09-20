use std::{
    boxed::Box,
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

use num::Rational64;
use rand::random;

mod list;
mod map;
mod set;
mod traits;
mod vector;

use traits::callable;

#[derive(Debug, Clone)]
pub enum Value {
    DefMacro,
    Fn(Box<dyn callable::Callable>),
    Error(String),

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
            (Self::DefMacro, Self::DefMacro) => true,
            (Self::Fn(_), Self::Fn(_)) => false,
            (Self::Error(_), Self::Error(_)) => false,
            (Self::List(l1), Self::List(l2)) => l1 == l2,
            (Self::Vector(v1), Self::Vector(v2)) => v1 == v2,
            (Self::Set(s1), Self::Set(s2)) => s1 == s2,
            (Self::Map(m1), Self::Map(m2)) => m1 == m2,
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
            Self::Error(e) => format!("#error \"{}\"", e),
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
enum ValueHash {
    DefMacro,
    Nil,
}
impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::DefMacro => ValueHash::DefMacro.hash(state),
            Value::Fn(_) => {
                let x: u16 = random();
                x.hash(state)
            }
            Value::Error(s) => s.hash(state),
            Value::List(l) => l.hash(state),
            Value::Vector(v) => v.hash(state),
            Value::Set(s) => s.hash(state),
            Value::Map(m) => m.hash(state),
            Value::Identifier(i) => i.hash(state),
            Value::String(s) => s.hash(state),
            Value::Number(n) => n.hash(state),
            Value::Nil => ValueHash::Nil.hash(state),
        }
    }
}
