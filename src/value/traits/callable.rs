use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::value::Value;

pub trait Callable: Debug + DynClone {
    fn call(&self, args: &[Value]) -> Value;
}

dyn_clone::clone_trait_object!(Callable);
