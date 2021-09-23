pub mod collectionfns;
pub mod comparisonops;
pub mod factorops;
pub mod iofns;

pub use comparisonops::ComparisonOp;
pub use factorops::FactorOp;

use std::fmt::{Debug, Display};

use dyn_clone::DynClone;

use crate::value::Value;

pub trait Callable: Display + Debug + DynClone {
    fn call(&self, args: &[Value]) -> Value;

    fn equals(&self, other: &dyn Callable) -> bool {
        format!("{}", self) == format!("{}", other)
    }
}

dyn_clone::clone_trait_object!(Callable);
