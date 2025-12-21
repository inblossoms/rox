use crate::parser::AST;

pub struct OUTPUT {}
pub type Error = ();
pub fn evaluate(ast: AST) -> Result<OUTPUT, Error> {
    println!("Evaluating...");
    Ok(OUTPUT {})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!((), ());
    }
}
