use crate::{
    ast::Expr,
    parser::{error::Error, parse::ParseHelper},
    tokenizer::{Literal, TokenType},
};

impl ParseHelper {
    /// Call: func(arg1, arg2)
    ///
    /// 解析函数调用表达式，支持多参数函数调用
    pub fn parse_call(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_token(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    /// 解析参数列表
    ///
    /// 函数调用解析，处理参数列表
    ///
    /// # 参数
    /// * `callee` - 被调用的函数表达式
    pub fn finish_call(&mut self, callee: Expr) -> Result<Expr, Error> {
        let mut args = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                // 参数可以是 任意表达式
                args.push(self.parse_expression()?);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        // 匹配右括号
        self.consume(TokenType::RightParen, "Expected ')' after arguments.")?;

        let name = match callee {
            Expr::Variable { name } => name,
            // 如果你的语言支持闭包或复杂调用 (e.g. getFunc()())，这里需要修改 Expr::Call 的定义
            // 假设 Expr::Call 目前只支持名字调用
            _ => return Err(self.error(self.previous(), "Can only call identifiers.")),
        };

        Ok(Expr::Call { name, args })
    }

    /// 基本表达式 (Primary)
    ///
    /// 解析基本表达式，包括字面量、标识符和分组表达式
    pub fn parse_primary(&mut self) -> Result<Expr, Error> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::Boolean { value: false });
        }
        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::Boolean { value: true });
        }
        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::Nil);
        }

        if self.match_token(&[TokenType::Number]) {
            let value = match &self.previous().literal {
                Literal::Number(n) => n.to_string(),
                _ => "0".to_string(),
            };
            return Ok(Expr::Number { value });
        }

        if self.match_token(&[TokenType::String]) {
            let value = match &self.previous().literal {
                Literal::String(s) => s.clone(),
                _ => "".to_string(),
            };
            return Ok(Expr::String { value });
        }

        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Expr::Variable {
                name: self.previous().lexeme.clone(),
            });
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.parse_expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
            return Ok(Expr::Grouping {
                expr: Box::new(expr),
            });
        }

        let current_token = self.peek();
        Err(self.error(current_token, &format!(
            "Unexpected token '{:?}'. Expected a primary expression (boolean, number, string, identifier, or grouping expression).",

            current_token.token_type
        )))
    }
}
