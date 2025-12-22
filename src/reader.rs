use std::fs::read_to_string;

#[derive(Debug)]
pub struct Source {
    pub content: String,
}

#[derive(Debug)]
pub struct Error {}
pub fn reader_source(filename: &str) -> Result<Source, Error> {
    println!("Reading source code...");
    let content = read_to_string(filename).expect("Error reading file");
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
