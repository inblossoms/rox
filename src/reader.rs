pub struct Source {}

#[derive(Debug)]
pub struct Error {}
pub fn reader_source(filename: &str) -> Result<Source, Error> {
    println!("Reading source code...");
    Ok(Source {})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!((), ());
    }
}
