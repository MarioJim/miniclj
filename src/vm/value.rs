use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet, VecDeque},
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

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
            Value::String(_) => "a string",
            Value::Number(_) => "a number",
            Value::Nil => "nil",
        }
    }

    pub fn as_int(&self) -> Result<i64, &'static str> {
        if let Value::Number(n) = self {
            if n.is_integer() {
                Ok(n.to_integer())
            } else {
                Err("a fraction")
            }
        } else {
            Err(self.type_str())
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
    fn from(constant: Constant) -> Value {
        match constant {
            Constant::Callable(c) => Value::Callable(c),
            Constant::Lambda(ptr, arity) => Value::Lambda(ptr, arity),
            Constant::String(s) => Value::String(s),
            Constant::Number(n) => Value::Number(n),
            Constant::Nil => Value::Nil,
        }
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Value {
        Value::Number(Rational64::from(n))
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Value {
        if b {
            Value::Number(Rational64::from(1))
        } else {
            Value::Number(Rational64::from(0))
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

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Callable(c) => write!(f, "{}", c.name()),
            Value::Lambda(ptr, _) => write!(f, "fn@{}", ptr),
            Value::List(l) => {
                let string = l
                    .iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(f, "({})", string)
            }
            Value::Vector(v) => {
                let string = v
                    .iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(f, "[{}]", string)
            }
            Value::Set(s) => {
                let string = s
                    .iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(f, "[{}]", string)
            }
            Value::Map(m) => {
                let string = m
                    .iter()
                    .map(|(k, v)| format!("[{}, {}]", k, v))
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(f, "{{{}}}", string)
            }
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Number(n) => write!(f, "{}", n),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Callable(c1), Value::Callable(c2)) => c1.name() == c2.name(),
            (Value::Lambda(ptr1, _), Value::Lambda(ptr2, _)) => ptr1 == ptr2,
            (Value::List(l1), Value::List(l2)) => l1 == l2,
            (Value::Vector(v1), Value::Vector(v2)) => v1 == v2,
            (Value::Set(s1), Value::Set(s2)) => s1 == s2,
            (Value::Map(m1), Value::Map(m2)) => m1 == m2,
            (Value::String(s1), Value::String(s2)) => s1 == s2,
            (Value::Number(n1), Value::Number(n2)) => n1 == n2,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }
}
impl Eq for Value {}

#[derive(Hash)]
struct NilHash;

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Callable(c) => c.name().hash(state),
            Value::Lambda(ptr, _) => ptr.hash(state),
            Value::List(l) => l.hash(state),
            Value::Vector(v) => v.hash(state),
            Value::Set(s) => {
                let hash = s
                    .iter()
                    .map(|v| {
                        let mut inner_state = DefaultHasher::new();
                        v.hash(&mut inner_state);
                        inner_state.finish()
                    })
                    .fold(0, u64::wrapping_add);

                state.write_u64(hash);
            }
            Value::Map(m) => {
                let hash = m
                    .iter()
                    .map(|kv| {
                        let mut inner_state = DefaultHasher::new();
                        kv.hash(&mut inner_state);
                        inner_state.finish()
                    })
                    .fold(0, u64::wrapping_add);

                state.write_u64(hash);
            }
            Value::String(s) => s.hash(state),
            Value::Number(n) => n.hash(state),
            Value::Nil => NilHash.hash(state),
        }
    }
}