use std::{
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

use num::Rational64;

#[derive(Debug, Clone)]
pub enum Literal {
    Symbol(String),
    String(String),
    Number(Rational64),
    Nil,
}

impl Literal {
    pub fn type_str(&self) -> &'static str {
        match self {
            Literal::Symbol(_) => "a symbol",
            Literal::String(_) => "a string",
            Literal::Number(_) => "a number",
            Literal::Nil => "nil",
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::Symbol(r), Literal::Symbol(l)) => r == l,
            (Literal::String(r), Literal::String(l)) => r == l,
            (Literal::Number(r), Literal::Number(l)) => r == l,
            (Literal::Nil, Literal::Nil) => true,
            _ => false,
        }
    }
}
impl Eq for Literal {}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = match self {
            Literal::Symbol(s) => s.to_string(),
            Literal::String(s) => format!("\"{}\"", s),
            Literal::Number(n) => n.to_string(),
            Literal::Nil => String::from("nil"),
        };
        write!(f, "{}", string)
    }
}

#[derive(Hash)]
struct NilHash;

impl Hash for Literal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Literal::Symbol(s) => s.hash(state),
            Literal::String(s) => s.hash(state),
            Literal::Number(n) => n.hash(state),
            Literal::Nil => NilHash.hash(state),
        }
    }
}

impl From<i64> for Literal {
    fn from(n: i64) -> Self {
        Literal::Number(Rational64::from_integer(n))
    }
}

impl From<bool> for Literal {
    fn from(b: bool) -> Self {
        Literal::from(if b { 1 } else { 0 })
    }
}
