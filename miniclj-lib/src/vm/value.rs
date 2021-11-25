use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
};

use num::{Rational64, Zero};

use crate::{
    callables::Callable,
    constant::Constant,
    instruction::InstructionPtr,
    vm::{List, RuntimeError, RuntimeResult},
};

/// Represents a value used during execution of `miniclj` code
#[derive(Clone)]
pub enum Value {
    Callable(Box<dyn Callable>),
    Lambda(InstructionPtr, usize),

    List(List),
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

    pub fn as_i64(&self) -> Result<i64, &'static str> {
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

    pub fn as_usize(&self) -> Result<usize, &'static str> {
        self.as_i64()?.try_into().map_err(|_| "a negative number")
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

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Number(n) => !n.is_zero(),
            Value::Nil => false,
            _ => true,
        }
    }

    pub fn into_map_entry(self) -> RuntimeResult<(Value, Value)> {
        match self {
            Value::Vector(v) if v.len() == 2 => {
                let mut v_iter = v.into_iter();
                let key = v_iter.next().unwrap();
                let val = v_iter.next().unwrap();
                Ok((key, val))
            }
            _ => Err(RuntimeError::InvalidMapEntry),
        }
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

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Callable(c) => write!(f, "fn_{}", c.name()),
            Value::Lambda(ptr, _) => write!(f, "fn@{}", ptr),
            Value::List(l) => write!(f, "'{}", l),
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
                write!(f, "#{{{}}}", string)
            }
            Value::Map(m) => {
                let string = m
                    .iter()
                    .map(|(k, v)| format!("{} {}", k, v))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{{{}}}", string)
            }
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Number(n) => write!(f, "{}", n),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Callable(c) => write!(f, "{}", c.name()),
            Value::Lambda(..) => write!(f, "{:?}", self),
            Value::List(l) => write!(f, "{}", l),
            Value::Vector(..) => write!(f, "{:?}", self),
            Value::Set(..) => write!(f, "{:?}", self),
            Value::Map(..) => write!(f, "{:?}", self),
            Value::String(s) => write!(f, "{}", s),
            Value::Number(n) => {
                if n.denom() == &1 {
                    write!(f, "{}", n.numer())
                } else {
                    write!(f, "{}/{}", n.numer(), n.denom())
                }
            }
            Value::Nil => write!(f, ""),
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
