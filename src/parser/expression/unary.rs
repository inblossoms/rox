use crate::{
    ast::{Expr, Operator},
    parser::{error::Error, parse::ParseHelper},
    tokenizer::{Token, TokenType, Tokens},
};

impl ParseHelper {
    /// 一元运算 (Unary): !, -
    pub fn parse_unary(&mut self) -> Result<Expr, Error> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let op = match self.previous().token_type {
                TokenType::Minus => Operator::Sub,
                TokenType::Bang => Operator::Not,
                _ => unreachable!(),
            };
            // Note：
            // 一元运算符的操作数是 unary 自身（eg: -1、!var、!!true）或更高优先级的项
            // 所以不能用 parse_expression，否则会吞掉后续的二元运算
            let right = self.parse_unary()?;
            return Ok(Expr::Unary {
                op,
                expr: Box::new(right),
            });
        }

        self.parse_call()
    }
}
