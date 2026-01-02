use std::io;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}
