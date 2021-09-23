use std::{
    collections::VecDeque,
    convert::TryInto,
    fmt::{self, Display, Formatter},
};

use num::{Rational64, Signed};

use crate::value::Value;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct List(VecDeque<Value>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = self
            .0
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "({})", string)
    }
}

impl List {
    pub fn rest(&self) -> Value {
        let mut new_list = self.0.clone();
        new_list.pop_front();
        Value::List(List(new_list))
    }

    pub fn cons(&self, val: Value) -> Value {
        let mut cloned_list = self.0.clone();
        cloned_list.push_front(val);
        Value::List(List(cloned_list))
    }

    pub fn get(&self, key: &Value) -> Value {
        if let Value::Number(n) = key {
            if n.is_integer() && !n.is_negative() {
                if n < &Rational64::from_integer(self.len().try_into().unwrap()) {
                    let index = (*n.numer()).try_into().unwrap();
                    self.0.get(index).unwrap_or(&Value::Nil).clone()
                } else {
                    Value::Nil
                }
            } else {
                Value::Error(format!(
                    "List can only be indexed by positive integers, not by {}",
                    n
                ))
            }
        } else {
            Value::Error(format!("List can't be indexed by {}", key))
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
