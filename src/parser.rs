use crate::tokenizer::Tokens;

pub type AST = ();

pub fn parse(tokens: Tokens) -> AST {
    println!("Parsing...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!((), ());
    }
}
