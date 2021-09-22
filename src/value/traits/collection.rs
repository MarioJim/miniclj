use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::value::Value;

pub trait Collection: Debug + DynClone {
    fn cons(&self, val: Value) -> Value;
    fn get(&self, key: &Value) -> Value;
    fn len(&self) -> usize;
}

dyn_clone::clone_trait_object!(Collection);
