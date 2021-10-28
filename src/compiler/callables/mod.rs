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
pub mod scopefns;
pub mod seqtransformfns;
pub mod typecastingfns;

pub use comparisonops::ComparisonOp;
pub use factorops::FactorOp;

use std::fmt::{Debug, Display};

use dyn_clone::DynClone;

use crate::compiler::{CompilationError, CompilationResult, SExpr, State};

pub trait Callable: Display + Debug + DynClone {
    fn name(&self) -> &'static str;
    fn compile(&self, state: &mut State, args: Vec<SExpr>) -> CompilationResult;

    fn arity_err(&self, expected: &'static str) -> CompilationResult {
        Err(CompilationError::ArityError(self.name(), expected))
    }

    fn is_user_defined(&self) -> bool {
        false
    }
}

dyn_clone::clone_trait_object!(Callable);
