use num::Rational64;

use crate::compiler::{memaddress::DataType, Literal};

#[derive(Debug, Clone)]
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
