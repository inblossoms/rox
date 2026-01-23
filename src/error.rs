use std::fmt;

use crate::evaluate;
use crate::parser;
use crate::reader;
use crate::tokenizer;

type Read = reader::Error;
type Parse = parser::Error;
type Tokenize = tokenizer::Error;
type Evaluate = evaluate::error::RuntimeError;
type Readline = rustyline::error::ReadlineError;

#[derive(Debug)]
#[allow(dead_code)]
pub enum RoxError {
    Read(Read),
    Parse(Parse),
    Evaluate(Evaluate),
    Tokenize(Tokenize),
    Readline(Readline),
}

impl fmt::Display for RoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoxError::Read(error) => write!(f, "{}", error),
            RoxError::Parse(error) => write!(f, "{}", error),
            RoxError::Evaluate(error) => write!(f, "{}", error),
            RoxError::Tokenize(error) => write!(f, "{}", error),
            RoxError::Readline(e) => write!(f, "Readline Error: {}", e),
        }
    }
}

macro_rules! impl_from_error {
    ($enum_name:ident, $($variant:ident),+) => {
        $(
            impl From<$variant> for $enum_name {
                fn from(error: $variant) -> Self {
                    Self::$variant(error)
                }
            }
        )+
    };
}

impl_from_error!(RoxError, Read, Parse, Evaluate, Tokenize, Readline);
