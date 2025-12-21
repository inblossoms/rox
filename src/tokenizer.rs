use crate::reader::Source;

pub struct Tokens {}

#[derive(Debug)]
pub struct Error {}
pub fn tokenize(source: Source) -> Result<Tokens, Error> {
    println!("Tokenizing...");
    Ok(Tokens {})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!((), ());
    }
}
