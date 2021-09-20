use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

use num::Rational64;

use crate::value::{traits::collection::Collection, Value};

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

impl Collection for Map {
    fn cons(&mut self, entry: Value) {
        if let Value::Vector(v) = entry {
            if v.len() == 2 {
                let key_idx = Value::Number(Rational64::from_integer(0));
                let val_idx = Value::Number(Rational64::from_integer(1));
                self.0.insert(v.get(&key_idx), v.get(&val_idx));
            }
        }
    }

    fn get(&self, key: &Value) -> Value {
        self.0.get(key).unwrap_or(&Value::Nil).clone()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn empty(&self) -> bool {
        self.0.is_empty()
    }
}
