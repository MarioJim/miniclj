use std::{
    convert::TryInto,
    fmt::{self, Display, Formatter},
};

use num::{Rational64, Signed};

use crate::{
    callables::{ExecutionResult, RuntimeError},
    value::Value,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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
    pub fn rest(&self) -> Value {
        let mut new_vector = self.0.iter();
        new_vector.next();
        Value::Vector(Vector(new_vector.cloned().collect()))
    }

    pub fn cons(&self, val: Value) -> Value {
        let mut cloned_vector = self.0.clone();
        cloned_vector.push(val);
        Value::Vector(Vector(cloned_vector))
    }

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
}
