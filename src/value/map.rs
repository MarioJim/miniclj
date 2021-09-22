use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

use num::Rational64;

use crate::value::Value;

#[derive(Debug, Eq, Clone)]
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
    pub fn cons(&self, entry: Value) -> Value {
        match entry {
            Value::Vector(v) if v.len() == 2 => {
                let key_idx = Value::Number(Rational64::from_integer(0));
                let val_idx = Value::Number(Rational64::from_integer(1));

                let mut cloned_map = self.0.clone();
                cloned_map.insert(v.get(&key_idx), v.get(&val_idx));
                Value::Map(Map(cloned_map))
            }
            _ => Value::Error(String::from(
                "Only vectors with two elements (key-value pair) can be added to a map",
            )),
        }
    }

    pub fn get(&self, key: &Value) -> Value {
        self.0.get(key).unwrap_or(&Value::Nil).clone()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
