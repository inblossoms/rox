#![allow(dead_code)]
use super::error::Error;
use std::fs::read_to_string;

#[derive(Debug)]
pub struct Source {
    pub contents: String,
}

pub fn reader_source(filename: &str) -> Result<Source, Error> {
    let file_extension = filename.split('.').last().unwrap();
    if file_extension != "rox" {
        return Err(Error::InvalidFileExtension(file_extension.to_string()));
    }
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
