use crate::{
    ast::{Expr, Operator},
    parser::{error::Error, parse::ParseHelper},
    tokenizer::TokenType,
};

impl ParseHelper {
    /// 相等性 (Equality): ==, !=
    pub fn parse_equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_comparison()?;

        while self.match_token(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let op = match self.previous().token_type {
                TokenType::EqualEqual => Operator::Equal,
                TokenType::BangEqual => Operator::NotEqual,
                _ => unreachable!(),
            };
            let right = self.parse_comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// 比较 (Comparison): <, <=, >, >=
    pub fn parse_comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_bitwise_or()?;

        while self.match_token(&[
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ]) {
            let op = match self.previous().token_type {
                TokenType::Less => Operator::Less,
                TokenType::LessEqual => Operator::LessEqual,
                TokenType::Greater => Operator::Greater,
                TokenType::GreaterEqual => Operator::GreaterEqual,
                _ => unreachable!(),
            };
            let right = self.parse_bitwise_or()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// 按位或 (|)
    /// 优先级：低于 &，高于 comparison
    pub fn parse_bitwise_or(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_bitwise_xor()?;

        while self.match_token(&[TokenType::Pipe]) {
            let op = Operator::BitwiseOr;
            let right = self.parse_bitwise_xor()?; // 右结合性交给循环，右侧调用下一层级
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// 按位异或 (^)
    /// 优先级：低于 &，高于 |
    pub fn parse_bitwise_xor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_bitwise_and()?;

        while self.match_token(&[TokenType::Xor]) {
            let op = Operator::BitwiseXor;
            let right = self.parse_bitwise_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// 按位与 (&)
    /// 优先级：低于 +，高于 |
    pub fn parse_bitwise_and(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_term()?; // 调用下一层级：加减法

        while self.match_token(&[TokenType::Ampersand]) {
            let op = Operator::BitwiseAnd;
            let right = self.parse_term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// 项 (Term): +, -
    pub fn parse_term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_factor()?;

        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let op = match self.previous().token_type {
                TokenType::Plus => Operator::Add,
                TokenType::Minus => Operator::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// 因子 (Factor): *, /, %
    pub fn parse_factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_unary()?;

        while self.match_token(&[TokenType::Star, TokenType::Slash, TokenType::Percent]) {
            let op = match self.previous().token_type {
                TokenType::Star => Operator::Mul,
                TokenType::Slash => Operator::Div,
                TokenType::Percent => Operator::Mod,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }
}
