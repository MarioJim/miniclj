use std::fmt::{self, Display, Formatter};

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Debug)]
pub enum RuntimeError {
    CompilerError(String),
    CouldntParse(String, &'static str),
    DivisionByZero,
    IndexOutOfBounds(&'static str),
    InvalidMapEntry,
    IOError(&'static str, std::io::Error),
    NotACallable(&'static str),
    WrongArityN(&'static str, usize, usize),
    WrongArityS(&'static str, &'static str, usize),
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
