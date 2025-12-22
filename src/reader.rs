use std::{fs::read_to_string, io};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Source {
    pub content: String,
}

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

pub fn reader_source(filename: &str) -> Result<Source, Error> {
    println!("Reading source code...");
    let content = read_to_string(filename)?;
    Ok(Source { content })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!((), ());
    }
}
