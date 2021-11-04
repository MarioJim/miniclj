use std::fmt::{self, Display};

pub type RuntimeResult = Result<(), RuntimeError>;

pub enum RuntimeError {
    WrongDataType(&'static str, &'static str, &'static str),
    NotACallable(&'static str),
    WrongArity(&'static str, usize, usize),
    IndexOutOfBounds(&'static str),
    DivisionByZero,
    CompilerError(String),
    Error(String),
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::WrongDataType(callable, expect, got) => write!(
                f,
                "Callable {} called with wrong argument, expected {}, got {}",
                callable, expect, got
            ),
            RuntimeError::NotACallable(value_type) => {
                write!(f, "Couldn't execute {} as a callable", value_type)
            }
            RuntimeError::WrongArity(callable, expect, got) => write!(
                f,
                "{} called with wrong number of arguments, expected {}, got {}",
                callable, expect, got
            ),
            RuntimeError::IndexOutOfBounds(value_type) => {
                write!(f, "Index of out bounds while indexing {}", value_type)
            }
            RuntimeError::DivisionByZero => write!(f, "Division by zero"),
            RuntimeError::CompilerError(err) => write!(f, "Compiler error: {}", err),
            RuntimeError::Error(err) => write!(f, "{}", err),
        }
    }
}
