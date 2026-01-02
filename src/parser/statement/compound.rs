use crate::{
    ast::Expr,
    parser::{error::Error, parse::ParseHelper},
    tokenizer::TokenType,
};

// 处理通过语句行为
impl ParseHelper {
    /// 解析代码块语句
    pub fn parse_block(&mut self) -> Result<Vec<Expr>, Error> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    /// 解析打印语句
    pub fn parse_print_statement(&mut self) -> Result<Expr, Error> {
        let expr = self.parse_expression_statement()?;
        Ok(Expr::Print {
            expr: Box::new(expr),
        })
    }
}
