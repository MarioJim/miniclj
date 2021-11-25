use std::fmt::{self, Display, Formatter};

pub type RuntimeResult<T> = Result<T, RuntimeError>;

/// Represents the different errors that can happen during runtime
#[derive(Debug)]
pub enum RuntimeError {
    /// This variant of `RuntimeError` encloses any error that
    /// was caused by a compiler malfunction and should only be
    /// encountered by the user if the compiler has a bug or
    /// if the bytecode was modified
    CompilerError(String),
    /// This variant is returned when a value that was passed
    /// to a parsing function (like `num` and `chr`) couldn't
    /// be correctly processed
    CouldntParse(String, &'static str),
    /// Returned when the user tries to divide a number by zero
    DivisionByZero,
    /// Returned when the user tries to get a value from
    /// an indexed collection using the callable `nth`
    /// and the collection is shorter than the index
    IndexOutOfBounds(&'static str),
    /// Returned when, inside a function, a value is implicitly
    /// casted to a map entry, but the value isn't a vector
    /// with two elements
    InvalidMapEntry,
    /// Returned when a input/output function returned an error
    /// instead of correctly printing/reading strings
    IOError(&'static str, std::io::Error),
    /// Returned when the user tried to execute a value
    /// as a callable, but it wasn't a language function
    /// nor a user-defined callable
    NotACallable(&'static str),
    /// Returned when the user tried to call a callable
    /// with the wrong number of arguments, variant for functions
    /// with a specific arity
    WrongArityN(&'static str, usize, usize),
    /// Returned when the user tried to call a callable
    /// with the wrong number of arguments, variant for functions
    /// that can be called with different numbers of arguments
    WrongArityS(&'static str, &'static str, usize),
    /// Returned when a callable receives a value with an incorrect
    /// datatype, that the callable didn't expect
    WrongDataType(&'static str, &'static str, &'static str),
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::CompilerError(err) => write!(f, "Compiler error: {}", err),
            RuntimeError::CouldntParse(string, expected) => {
                write!(f, "Couldn't parse the value {} to {}", string, expected)
            }
            RuntimeError::DivisionByZero => write!(f, "Division by zero"),
            RuntimeError::IndexOutOfBounds(value_type) => {
                write!(f, "Index of out bounds while indexing {}", value_type)
            }
            RuntimeError::InvalidMapEntry => write!(
                f,
                "Only vectors with two elements (key-value pair) can be added to a map"
            ),
            RuntimeError::IOError(context, error) => {
                write!(f, "Error trying to {}: {}", context, error)
            }
            RuntimeError::NotACallable(value_type) => {
                write!(f, "Couldn't execute {} as a callable", value_type)
            }
            RuntimeError::WrongArityN(callable, expect, got) => write!(
                f,
                "{} called with wrong number of arguments, expected {}, got {}",
                callable, expect, got
            ),
            RuntimeError::WrongArityS(callable, expect, got) => write!(
                f,
                "{} called with wrong number of arguments, expected {}, got {}",
                callable, expect, got
            ),
            RuntimeError::WrongDataType(callable, expect, got) => write!(
                f,
                "Callable {} called with wrong argument, expected {}, got {}",
                callable, expect, got
            ),
        }
    }
}
