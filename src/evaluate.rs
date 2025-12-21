use crate::parser::AST;

pub type OUTPUT = ();
pub fn evaluate(ast: AST) -> OUTPUT {
    println!("Evaluating...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!((), ());
    }
}
