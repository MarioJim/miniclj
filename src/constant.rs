use std::fmt::{self, Display, Formatter};

use num::Rational64;

use crate::{compiler::Literal, memaddress::DataType};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Constant {
    String(String),
    Number(Rational64),
    Nil,
}

impl Constant {
    pub fn data_type(&self) -> DataType {
        match self {
            Constant::String(_) => DataType::String,
            Constant::Number(_) => DataType::Number,
            Constant::Nil => DataType::Nil,
        }
    }
}

impl From<Literal> for Constant {
    fn from(literal: Literal) -> Self {
        match literal {
            Literal::Symbol(_) => panic!("Trying to convert a symbol literal to a constant"),
            Literal::String(s) => Constant::String(s),
            Literal::Number(n) => Constant::Number(n),
            Literal::Nil => Constant::Nil,
        }
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Constant::String(string) => write!(f, "\"{}\"", string),
            Constant::Number(num) => write!(f, "{}/{}", num.numer(), num.denom()),
            Constant::Nil => write!(f, "nil"),
        }
    }
}
