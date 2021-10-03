use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

use crate::{callables::ExecutionResult, value::Value};

#[derive(Debug, Default, Eq, Clone)]
pub struct Map(HashMap<Value, Value>);

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = self
            .0
            .iter()
            .map(|(k, v)| format!("[{}, {}]", k, v))
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{{{}}}", string)
    }
}

impl Hash for Map {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let s = self
            .0
            .iter()
            .map(|kv| {
                let mut inner_state = DefaultHasher::new();
                kv.hash(&mut inner_state);
                inner_state.finish()
            })
            .fold(0, u64::wrapping_add);

        state.write_u64(s);
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Map {
    pub fn get(&self, key: &Value) -> ExecutionResult {
        Ok(self.0.get(key).cloned().unwrap_or(Value::Nil))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: Value, val: Value) {
        self.0.insert(key, val);
    }
}

pub type MapIter = std::collections::hash_map::IntoIter<Value, Value>;

impl IntoIterator for Map {
    type Item = (Value, Value);
    type IntoIter = MapIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
