use std::{
    collections::VecDeque,
    convert::TryInto,
    fmt::{self, Display, Formatter},
    iter::FromIterator,
};

use num::{Rational64, Signed};

use crate::compiler::{
    callables::{ExecutionResult, RuntimeError},
    value::Value,
};

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
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

    pub fn get(&self, key: &Value) -> ExecutionResult {
        if let Value::Number(n) = key {
            if n.is_integer() && !n.is_negative() {
                if n < &Rational64::from_integer(self.len().try_into().unwrap()) {
                    let index = (*n.numer()).try_into().unwrap();
                    Ok(self.0.get(index).unwrap_or(&Value::Nil).clone())
                } else {
                    Ok(Value::Nil)
                }
            } else {
                Err(RuntimeError::Error(format!(
                    "List can only be indexed by positive integers, not by {}",
                    n
                )))
            }
        } else {
            Err(RuntimeError::Error(format!(
                "List can't be indexed by {}",
                key
            )))
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push_front(&mut self, value: Value) {
        self.0.push_front(value)
    }

    pub fn push_back(&mut self, value: Value) {
        self.0.push_back(value)
    }

    pub fn pop_front(&mut self) -> Option<Value> {
        self.0.pop_front()
    }
}

pub type ListIter = std::collections::vec_deque::IntoIter<Value>;

impl IntoIterator for List {
    type Item = Value;
    type IntoIter = ListIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Value> for List {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        List(iter.into_iter().collect())
    }
}
