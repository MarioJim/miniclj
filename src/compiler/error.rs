use std::fmt::{self, Display};

use crate::memaddress::MemAddress;

pub type CompilationResult = Result<MemAddress, CompilationError>;

#[derive(Debug)]
pub enum CompilationError {
    CallableNotDefined(String),
    EmptyArgs(&'static str),
    SymbolNotDefined(String),
    WrongArgument(&'static str, &'static str, &'static str),
    WrongArity(&'static str, &'static str),
    WrongRecurCall(usize, usize),
}

impl Display for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilationError::CallableNotDefined(callable_name) => {
                write!(
                    f,
                    "Callable \"{}\" not defined in the current scope",
                    callable_name
                )
            }
            CompilationError::EmptyArgs(callable) => write!(
                f,
                "Callable {} expected at least one argument, none were provided",
                callable
            ),
            CompilationError::SymbolNotDefined(symbol) => {
                write!(f, "Symbol \"{}\" not defined in the current scope", symbol)
            }
            CompilationError::WrongArgument(callable, expect, got) => write!(
                f,
                "Callable {} called with wrong argument, expected {}, got {}",
                callable, expect, got
            ),
            CompilationError::WrongArity(callable, args) => write!(
                f,
                "Callable {0} called with wrong number of arguments, should be called as ({0} {1})",
                callable, args
            ),
            CompilationError::WrongRecurCall(expected, got) => write!(
                f,
                "recur call expected {} arguments, got {} arguments",
                expected, got
            ),
        }
    }
}
