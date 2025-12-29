use crate::{
    ast::Expr,
    parser::{error::Error, parse::ParseHelper},
    tokenizer::TokenType,
};

impl ParseHelper {
    pub fn parse_if_statement(&mut self) -> Result<Expr, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        // 递归调用 parse_statement 以支持单行语句或Block
        let then_branch = self.parse_statement()?;

        // 确保 else 绑定到最近的 if
        let else_branch = if self.match_token(&[TokenType::Else]) {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(Expr::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    pub fn parse_while_statement(&mut self) -> Result<Expr, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = self.parse_statement()?;

        Ok(Expr::While {
            condition: Box::new(condition),
            body: Box::new(body),
        })
    }
}
