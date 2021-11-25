use std::fmt::{self, Display};

use smol_str::SmolStr;

use crate::memaddress::MemAddress;

pub type CompilationResult = Result<MemAddress, CompilationError>;

/// Represents the type of errors generated during compilation
#[derive(Debug)]
pub enum CompilationError {
    /// Returned when the compiler finds a symbol that was supposed
    /// to be used as a callable, but isn't defined in the current
    /// scope (wasn't a user-defined function nor a language callable)
    CallableNotDefined(SmolStr),
    /// Returned when a expression tried to call a callable with
    /// no arguments, and the callalbe expects at least one
    EmptyArgs(&'static str),
    /// Returned by the compiler when a symbol wasn't defined
    /// in the current scope (or any other parent scope)
    SymbolNotDefined(SmolStr),
    /// Returned by the compiler when a function receives an argument
    /// that it didn't expect. Although most functions don't check the
    /// type of its arguments during compilation, some functions with
    /// a custom compilation process (such as `fn`, `defn` and `let`)
    /// use their arguments during compilation
    WrongArgument(&'static str, &'static str, &'static str),
    /// Returned when the user tried to call a callable with
    /// the wrong number of arguments
    WrongArity(&'static str, &'static str),
    /// Returned when the user tried to call the `recur` callable
    /// with a different number of arguments than it's corresponding
    /// `loop` call
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
