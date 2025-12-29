use crate::{
    ast::Expr,
    parser::{error::Error, parse::ParseHelper},
    tokenizer::{Token, TokenType, Tokens},
};

impl ParseHelper {
    pub fn parse_var_declaration(&mut self) -> Result<Expr, Error> {
        let name_token = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let name = name_token.lexeme.clone();

        let initializer = if self.match_token(&[TokenType::Equal]) {
            self.parse_expression()?
        } else {
            Expr::Nil
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Expr::Assign {
            name,
            expr: Box::new(initializer),
        })
    }

    pub fn parse_function_declaration(&mut self) -> Result<Expr, Error> {
        let name_token = self.consume(TokenType::Identifier, "Expect function name.")?;
        let name = name_token.lexeme.clone();

        self.consume(TokenType::LeftParen, "Expect '(' after function name.")?;
        let mut args = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                let arg_token = self.consume(TokenType::Identifier, "Expect parameter name.")?;
                args.push(arg_token.lexeme.clone());
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(TokenType::LeftBrace, "Expect '{' before function body.")?;
        let body_stmts = self.parse_block()?;

        Ok(Expr::Function {
            name,
            args,
            body: body_stmts,
        })
    }

    pub fn parse_block(&mut self) -> Result<Vec<Expr>, Error> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }
}
