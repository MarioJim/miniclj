use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

use crate::{callables::ExecutionResult, value::Value};

#[derive(Debug, Default, Eq, Clone)]
pub struct Set(HashSet<Value>);

impl Display for Set {
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

impl Hash for Set {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let s = self
            .0
            .iter()
            .map(|v| {
                let mut inner_state = DefaultHasher::new();
                v.hash(&mut inner_state);
                inner_state.finish()
            })
            .fold(0, u64::wrapping_add);

        state.write_u64(s);
    }
}

impl PartialEq for Set {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Set {
    pub fn get(&self, key: &Value) -> ExecutionResult {
        Ok(Value::from(self.0.contains(key)))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, val: Value) {
        self.0.insert(val);
    }
}

pub type SetIter = std::collections::hash_set::IntoIter<Value>;

impl IntoIterator for Set {
    type Item = Value;
    type IntoIter = SetIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
