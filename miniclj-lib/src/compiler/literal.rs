use std::fmt::{self, Display, Formatter};

use num::Rational64;
use smol_str::SmolStr;

/// Represents a value extracted directly from a
/// s-expression and parsed by the `SExprParser`
#[derive(Debug, Clone)]
pub enum Literal {
    Symbol(SmolStr),
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
    fn eq(&self, other: &Literal) -> bool {
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
        match self {
            Literal::Symbol(s) => write!(f, "{}", s),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Nil => write!(f, "nil"),
        }
    }
}
