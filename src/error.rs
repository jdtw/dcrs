use std::convert::From;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    InvalidInput(String),
    EndOfStack,
    EndOfRegister(String),
    BadRadix,
    Io(io::Error),
    DivideByZero,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            InvalidInput(s) => write!(f, "Invalid input: '{}'", s),
            EndOfStack => write!(f, "Stack is empty"),
            EndOfRegister(id) => write!(f, "Register stack '{}' is empty", id.trim()),
            Io(error) => write!(f, "IO Error: {}", error),
            BadRadix => write!(f, "Bad radix"),
            DivideByZero => write!(f, "Attempt to divide by zero"),
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}
