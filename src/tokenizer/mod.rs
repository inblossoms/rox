mod error;
mod literal;
mod scanner;
mod token;
mod token_type;

pub use error::Error;
pub use literal::Literal;
pub use scanner::tokenize;
pub use token::Token;
pub use token_type::TokenType;

#[derive(Debug)]
pub struct Tokens {
    pub tokens: Vec<Token>,
}
