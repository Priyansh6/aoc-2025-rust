use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyInput,
    NotADigit(char),
    WrongLength {
        expected: usize,
        got: usize,
        input: String,
    },
    NotWrapped {
        open: String,
        close: String,
    },
    Other(String),
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ParseError::EmptyInput => write!(f, "unexpected empty input"),
            ParseError::NotADigit(c) => write!(f, "expected a digit, got '{c}'"),
            ParseError::WrongLength {
                expected,
                got,
                input,
            } => {
                write!(f, "expected {expected} items, got {got} in \"{input}\"")
            }
            ParseError::NotWrapped { open, close } => {
                write!(f, "expected input wrapped in {open} ... {close}")
            }
            ParseError::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl From<String> for ParseError {
    fn from(s: String) -> Self {
        ParseError::Other(s)
    }
}

impl From<&str> for ParseError {
    fn from(s: &str) -> Self {
        ParseError::Other(s.to_string())
    }
}
