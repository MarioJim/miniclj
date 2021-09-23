macro_rules! display_for_callable {
    ($callable:ty) => {
        impl std::fmt::Display for $callable {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.name())
            }
        }
    };
}

pub mod collectionfns;
pub mod comparisonops;
pub mod conditionals;
pub mod factorops;
pub mod groupingfns;
pub mod iofns;
pub mod lambdafns;
pub mod scopefns;
pub mod seqtransformfns;
pub mod typecastingfns;

pub use comparisonops::ComparisonOp;
pub use factorops::FactorOp;

use std::fmt::{self, Debug, Display};

use dyn_clone::DynClone;

use crate::{Scope, Value};

pub trait Callable: Display + Debug + DynClone {
    fn name(&self) -> &'static str;
    fn call(&self, args: &[Value], scope: &Scope) -> ExecutionResult;
}

dyn_clone::clone_trait_object!(Callable);

pub type ExecutionResult = Result<Value, RuntimeError>;

#[derive(Debug)]
pub enum RuntimeError {
    ArityError(&'static str, &'static str),
    WrongArgument(&'static str, &'static str, &'static str),
    DivisionByZero,
    GenericError(String),
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::ArityError(callable, args) => write!(
                f,
                "Callable {0} called with wrong number of arguments, should be called as ({0} {1})",
                callable, args
            ),
            RuntimeError::WrongArgument(callable, expect, got) => write!(
                f,
                "Callable {} called with wrong argument, expected {}, got {}",
                callable, expect, got
            ),
            RuntimeError::DivisionByZero => write!(f, "Division by zero is undefined behavior"),
            RuntimeError::GenericError(s) => write!(f, "{}", s),
        }
    }
}
