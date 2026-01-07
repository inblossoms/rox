use crate::{
    ast::Stmt,
    parser::{error::Error, parse::ParseHelper},
    tokenizer::TokenType,
};

// 处理通过语句行为
impl ParseHelper {
    /// 解析代码块语句
    pub fn parse_block(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    /// 解析打印语句
    pub fn parse_print_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.parse_expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::Print { expr })
    }

    pub fn parse_return_statement(&mut self) -> Result<Stmt, Error> {
        let keyword = self.previous().clone();

        if self.func_depth == 0 {
            self.error(&keyword, "Cannot return from top-level code.");
        }

        let value = if !self.check(TokenType::Semicolon) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;

        Ok(Stmt::Return { keyword, value })
    }
}
