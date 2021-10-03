use std::{
    convert::{TryFrom, TryInto},
    fmt::{self, Display, Formatter},
};

use num::{Rational64, Signed};

use crate::{
    callables::{ExecutionResult, RuntimeError},
    value::Value,
};

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
pub struct Vector(Vec<Value>);

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = self
            .0
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "[{}]", string)
    }
}

impl Vector {
    pub fn get(&self, key: &Value) -> ExecutionResult {
        if let Value::Number(n) = key {
            if n.is_integer() && !n.is_negative() {
                if n < &Rational64::from_integer(self.len().try_into().unwrap()) {
                    let index: usize = (*n.numer()).try_into().unwrap();
                    Ok(self.0.get(index).unwrap_or(&Value::Nil).clone())
                } else {
                    Ok(Value::Nil)
                }
            } else {
                Ok(Value::Nil)
            }
        } else {
            Err(RuntimeError::Error(format!(
                "Vector can't be indexed by {}",
                key
            )))
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, val: Value) {
        self.0.push(val)
    }
}

impl From<Vec<Value>> for Vector {
    fn from(v: Vec<Value>) -> Self {
        Vector(v)
    }
}

pub type VectorIter = std::vec::IntoIter<Value>;

impl IntoIterator for Vector {
    type Item = Value;
    type IntoIter = VectorIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl TryFrom<Vector> for (Value, Value) {
    type Error = ();

    fn try_from(vector: Vector) -> Result<Self, Self::Error> {
        if vector.len() == 2 {
            let mut it = vector.into_iter();
            let first = it.next().unwrap();
            let second = it.next().unwrap();
            Ok((first, second))
        } else {
            Err(())
        }
    }
}
