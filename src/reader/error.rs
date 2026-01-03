use std::{fmt::Display, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    InvalidFileExtension(String),
}

impl From<String> for Error {
    fn from(ext: String) -> Self {
        Self::InvalidFileExtension(ext)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "Could not read source file: {}", err),
            Error::InvalidFileExtension(ext) => {
                write!(f, "Invalid file extension: '{}'. Expecting '.rox'.", ext)
            }
        }
    }
}
