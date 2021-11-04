use std::fmt::{self, Display, Formatter};

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
        let string = match self {
            Literal::Symbol(s) => s.to_string(),
            Literal::String(s) => format!("\"{}\"", s),
            Literal::Number(n) => n.to_string(),
            Literal::Nil => String::from("nil"),
        };
        write!(f, "{}", string)
    }
}
