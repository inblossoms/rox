#![allow(dead_code)]
use std::{fs::read_to_string, io};

#[derive(Debug)]
pub struct Source {
    pub contents: String,
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

pub fn reader_source(filename: &str) -> Result<Source, Error> {
    println!("Reading source code...");
    let content = read_to_string(filename)?;
    Ok(Source { contents: content })
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!((), ());
    }
}
