use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

use num::Rational64;

use crate::value::{traits::collection::Collection, Value};

#[derive(Debug, Eq, Clone)]
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

impl Collection for Set {
    fn cons(&mut self, val: Value) {
        self.0.insert(val);
    }

    fn get(&self, key: &Value) -> Value {
        let v = if self.0.contains(key) { 1 } else { 0 };
        Value::Number(Rational64::from_integer(v))
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn empty(&self) -> bool {
        self.0.is_empty()
    }
}
