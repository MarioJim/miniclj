use std::{
    convert::TryInto,
    fmt::{self, Display, Formatter},
};

use num::{Rational64, Signed};

use crate::value::{traits::collection::Collection, Value};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Vector(Vec<Value>);

impl Collection for Vector {
    fn cons(&mut self, val: Value) {
        self.0.push(val);
    }

    fn get(&self, key: &Value) -> Value {
        if let Value::Number(n) = key {
            if n.is_integer() && !n.is_negative() {
                if n < &Rational64::from_integer(self.len().try_into().unwrap()) {
                    let index: usize = (*n.numer()).try_into().unwrap();
                    self.0.get(index).unwrap_or(&Value::Nil).clone()
                } else {
                    Value::Error(format!(
                        "Vector has {} elements, index {} out of bounds",
                        self.0.len(),
                        n
                    ))
                }
            } else {
                Value::Error(format!(
                    "Vector can only be indexed by positive integers, not by {}",
                    n
                ))
            }
        } else {
            Value::Error(format!("Vector can't be indexed by {}", key))
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn empty(&self) -> bool {
        self.0.is_empty()
    }
}

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
