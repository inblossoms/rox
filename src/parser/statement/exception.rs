use crate::{
    ast::Stmt,
    parser::{error::Error, parse::ParseHelper},
    tokenizer::TokenType,
};

impl ParseHelper {
    pub fn parse_try_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::LeftBrace, "Expect '{' after 'try'.")?;
        let try_body = self.parse_block()?;
        let try_branch = Stmt::Block { body: try_body };

        self.consume(TokenType::Catch, "Expect 'catch' after try block.")?;
        self.consume(TokenType::LeftParen, "Expect '(' after 'catch'.")?;
        let catch_var = self
            .consume(TokenType::Identifier, "Expect catch variable name.")?
            .clone();
        self.consume(TokenType::RightParen, "Expect ')' after catch variable.")?;

        self.consume(TokenType::LeftBrace, "Expect '{' after catch clause.")?;
        let catch_body = self.parse_block()?;
        let catch_branch = Stmt::Block { body: catch_body };

        Ok(Stmt::Try {
            try_branch: Box::new(try_branch),
            catch_var,
            catch_branch: Box::new(catch_branch),
        })
    }
}
