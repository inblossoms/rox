use std::fmt;

#[derive(Debug)]
pub struct Error {}

#[derive(Debug)]
#[allow(dead_code)]
pub enum RuntimeError {
    Generic(String),
    Catchable(super::Value),
    UndefinedVariable(String),
    TypeError(String),
    IndexError(String),
    ArgumentError(String),
    DivisionByZero,
    Return(super::Value),
    Print(String),
    Break,
    Continue,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeError::Generic(msg) => write!(f, "{}", msg),
            RuntimeError::Catchable(err) => write!(f, "{}", err),
            RuntimeError::UndefinedVariable(name) => write!(f, "Undefined variable '{}'.", name),
            RuntimeError::TypeError(msg) => write!(f, "Type error: {}", msg),
            RuntimeError::IndexError(msg) => write!(f, "Index error: {}", msg),
            RuntimeError::ArgumentError(msg) => write!(f, "Argument error: {}", msg),
            RuntimeError::DivisionByZero => write!(f, "Division by zero."),
            RuntimeError::Return(_) => write!(f, "Cannot 'return' from top-level code."),
            RuntimeError::Break => write!(f, "Cannot use 'break' outside of a loop."),
            RuntimeError::Continue => write!(f, "Cannot use 'continue' outside of a loop."),
            RuntimeError::Print(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An interpreter error occurred.")
    }
}
