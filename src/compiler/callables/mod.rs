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

use std::{
    fmt::{self, Debug, Display},
    rc::Rc,
};

use dyn_clone::DynClone;

use crate::compiler::{SExpr, Scope, State};

use super::state::Instruction;

pub trait Callable: Display + Debug + DynClone {
    fn name(&self) -> &'static str;
    fn compile(&self, state: &mut State, args: Vec<SExpr>, scope: &Rc<Scope>) -> CompilationResult;

    fn arity_err(&self, expected: &'static str) -> CompilationResult {
        Err(CompilationError::ArityError(self.name(), expected))
    }

    fn is_user_defined(&self) -> bool {
        false
    }
}

dyn_clone::clone_trait_object!(Callable);

pub type CompilationResult = Result<Vec<Instruction>, CompilationError>;

#[derive(Debug)]
pub enum CompilationError {
    ArityError(&'static str, &'static str),
    WrongArgument(&'static str, &'static str, &'static str),
    NotDefined(String),
    Error(String),
}

impl Display for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilationError::ArityError(callable, args) => write!(
                f,
                "Callable {0} called with wrong number of arguments, should be called as ({0} {1})",
                callable, args
            ),
            CompilationError::WrongArgument(callable, expect, got) => write!(
                f,
                "Callable {} called with wrong argument, expected {}, got {}",
                callable, expect, got
            ),
            CompilationError::NotDefined(symbol) => {
                write!(f, "Symbol \"{}\" not defined in the current scope", symbol)
            }
            CompilationError::Error(s) => write!(f, "{}", s),
        }
    }
}
