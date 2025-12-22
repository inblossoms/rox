use crate::tokenizer::Tokens;

pub struct AST {}

#[derive(Debug)]
pub struct Error {}

pub fn parse(_tokens: Tokens) -> Result<AST, Error> {
    println!("Parsing...");
    Ok(AST {})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!((), ());
    }
}
