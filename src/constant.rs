use std::{
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

use num::Rational64;

use crate::{callables::Callable, compiler::Literal, instruction::InstructionPtr};

#[derive(Debug, Clone)]
pub enum Constant {
    Callable(Box<dyn Callable>),
    Lambda(InstructionPtr),

    String(String),
    Number(Rational64),
    Nil,
}

impl Constant {
    pub fn new_lambda(instruction_ptr: InstructionPtr) -> Constant {
        Constant::Lambda(instruction_ptr)
    }
}

impl From<Box<dyn Callable>> for Constant {
    fn from(callable: Box<dyn Callable>) -> Self {
        Constant::Callable(callable)
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
            Constant::Callable(c) => write!(f, "{}", c.name()),
            Constant::Lambda(instruction_ptr) => write!(f, "fn@{}", instruction_ptr),
            Constant::String(string) => write!(f, "\"{}\"", string),
            Constant::Number(num) => write!(f, "{}/{}", num.numer(), num.denom()),
            Constant::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Hash)]
struct NilHash;

impl Hash for Constant {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Constant::Callable(c) => c.name().hash(state),
            Constant::Lambda(instr_ptr) => instr_ptr.hash(state),
            Constant::String(s) => s.hash(state),
            Constant::Number(n) => n.hash(state),
            Constant::Nil => NilHash.hash(state),
        }
    }
}

impl PartialEq for Constant {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Constant::Callable(c1), Constant::Callable(c2)) => c1.name() == c2.name(),
            (Constant::Lambda(ptr1), Constant::Lambda(ptr2)) => ptr1 == ptr2,
            (Constant::String(s1), Constant::String(s2)) => s1 == s2,
            (Constant::Number(n1), Constant::Number(n2)) => n1 == n2,
            (Constant::Nil, Constant::Nil) => true,
            _ => false,
        }
    }
}
impl Eq for Constant {}
