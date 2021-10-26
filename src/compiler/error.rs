use std::fmt::{self, Display};

use super::MemAddress;

pub type CompilationResult = Result<MemAddress, CompilationError>;

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
