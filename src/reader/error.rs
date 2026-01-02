use std::io;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    Io(io::Error),
    InvalidFileExtension(String),
}

impl Error {}

impl From<String> for Error {
    fn from(value: String) -> Self {
        let err_msg = format!("Invalid file extension: '{value}'. File must have '.rox' extension.");
        Self::InvalidFileExtension(err_msg)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}
