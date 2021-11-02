use std::fmt::{self, Display};

pub type RuntimeResult = Result<(), RuntimeError>;

pub enum RuntimeError {
    WrongDataType(&'static str, &'static str, &'static str),
    DivisionByZero,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::WrongDataType(callable, expect, got) => write!(
                f,
                "Callable {} called with wrong argument, expected {}, got {}",
                callable, expect, got
            ),
            RuntimeError::DivisionByZero => write!(f, "Division by zero"),
        }
    }
}
