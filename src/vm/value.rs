use std::collections::{HashMap, HashSet, VecDeque};

use num::Rational64;

use crate::{callables::Callable, constant::Constant, instruction::InstructionPtr};

#[derive(Debug, Clone)]
pub enum Value {
    Callable(Box<dyn Callable>),
    Lambda(InstructionPtr, usize),

    List(VecDeque<Value>),
    Vector(Vec<Value>),
    Set(HashSet<Value>),
    Map(HashMap<Value, Value>),

    Symbol(String),
    String(String),
    Number(Rational64),
    Nil,
}

impl Value {
    pub fn type_str(&self) -> &'static str {
        match self {
            Value::Callable(_) | Value::Lambda(..) => "a function",
            Value::List(_) => "a list",
            Value::Vector(_) => "a vector",
            Value::Set(_) => "a set",
            Value::Map(_) => "a map",
            Value::Symbol(_) => "a symbol",
            Value::String(_) => "a string",
            Value::Number(_) => "a number",
            Value::Nil => "nil",
        }
    }

    pub fn as_bool(&self) -> Result<bool, &'static str> {
        if let Value::Number(n) = self {
            if n == &Rational64::from(0) {
                return Ok(false);
            } else if n == &Rational64::from(1) {
                return Ok(true);
            }
        }
        Err(self.type_str())
    }
}

impl From<Constant> for Value {
    fn from(constant: Constant) -> Self {
        match constant {
            Constant::Callable(c) => Value::Callable(c),
            Constant::Lambda(ptr, arity) => Value::Lambda(ptr, arity),
            Constant::String(s) => Value::String(s),
            Constant::Number(n) => Value::Number(n),
            Constant::Nil => Value::Nil,
        }
    }
}

impl TryFrom<Value> for VecDeque<Value> {
    type Error = &'static str;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::List(l) => Ok(l),
            Value::Vector(v) => Ok(v.into_iter().collect()),
            Value::Set(s) => Ok(s.into_iter().collect()),
            Value::Map(m) => Ok(m
                .into_iter()
                .map(|(k, v)| Value::Vector(vec![k, v]))
                .collect()),
            Value::String(s) => Ok(s.chars().map(|c| Value::String(c.to_string())).collect()),
            _ => Err(value.type_str()),
        }
    }
}
