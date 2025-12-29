use crate::{
    ast::{AST, Expr, Operator},
    parser::error::Error,
    tokenizer::{Token, TokenType, Tokens},
};

#[derive(Debug)]
pub struct ParseHelper {
    pub tokens: Tokens,
    pub index: usize,
}

impl ParseHelper {
    //  (Helper Methods)

    pub fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    /// Current Token
    pub fn peek(&self) -> &Token {
        if self.index >= self.tokens.tokens.len() {
            // 返回最后一个 token (通常是 EOF)，防止越界 panic
            return &self.tokens.tokens[self.tokens.tokens.len() - 1];
        }
        &self.tokens.tokens[self.index]
    }

    /// Previous Token
    pub fn previous(&self) -> &Token {
        if self.index == 0 {
            return &self.tokens.tokens[0];
        }
        &self.tokens.tokens[self.index - 1]
    }

    /// 检查当前 Token 类型是否匹配
    pub fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() && token_type != TokenType::Eof {
            return false;
        }
        self.peek().token_type == token_type
    }

    /// 如果当前 Token 匹配指定类型，则消耗并返回 true
    pub fn match_token(&mut self, types: &[TokenType]) -> bool {
        for &t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// 消耗并返回当前 Token
    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.index += 1;
        }
        self.previous()
    }

    /// 强制要求当前 Token 为指定类型，否则返回错误
    pub fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, Error> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(self.error(self.peek(), message))
        }
    }

    pub fn error(&self, token: &Token, message: &str) -> Error {
        let error_message = format!("[line {}]: {}", token.line, message);

        Error {
            message: error_message,
            position: self.index,
        }
    }

    //  (Expression Parsing) - 按优先级从低到高

    /// 解析入口
    pub fn parse_expression(&mut self) -> Result<Expr, Error> {
        self.parse_assignment()
    }

    /// 赋值 (Assignment): variable = value
    pub fn parse_assignment(&mut self) -> Result<Expr, Error> {
        let expr = self.parse_or()?;

        if self.match_token(&[TokenType::Equal]) {
            let value = self.parse_assignment()?; // 递归，支持 a = b = 1

            // 检查左值是否合法
            if let Expr::Variable { name } = expr {
                return Ok(Expr::Assign {
                    name,
                    expr: Box::new(value),
                });
            }

            // 修复逻辑错误：显式报错无效的赋值目标
            return Err(self.error(self.previous(), "Invalid assignment target."));
        }

        Ok(expr)
    }

    /// OR
    pub fn parse_or(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_and()?;

        while self.match_token(&[TokenType::Or]) {
            let op = Operator::Or;
            let right = self.parse_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// AND
    pub fn parse_and(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_equality()?;

        while self.match_token(&[TokenType::And]) {
            let op = Operator::And;
            let right = self.parse_equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    // (Statement Parsing)

    pub fn parse_statement(&mut self) -> Result<Expr, Error> {
        if self.match_token(&[TokenType::If]) {
            return self.parse_if_statement();
        }
        if self.match_token(&[TokenType::While]) {
            return self.parse_while_statement();
        }
        if self.match_token(&[TokenType::Var]) {
            return self.parse_var_declaration();
        }
        if self.match_token(&[TokenType::Fun]) {
            return self.parse_function_declaration();
        }
        if self.match_token(&[TokenType::LeftBrace]) {
            let statements = self.parse_block()?;
            return Ok(Expr::Block { body: statements });
        }

        self.parse_expression_statement()
    }

    pub fn parse_expression_statement(&mut self) -> Result<Expr, Error> {
        let expr = self.parse_expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(expr)
    }

    // run
    pub fn parse_program(&mut self) -> Result<Expr, Error> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        Ok(Expr::Block { body: statements })
    }
}

#[derive(Debug)]
pub struct Parser {
    helper: ParseHelper,
}

impl Parser {
    pub fn new(tokens: Tokens) -> Self {
        Self {
            helper: ParseHelper { tokens, index: 0 },
        }
    }

    // run
    pub fn parse_program(&mut self) -> Result<Expr, Error> {
        self.helper.parse_program()
    }
}

///
pub fn parse(tokens: Tokens) -> Result<AST, Error> {
    let mut parser = Parser::new(tokens);
    let top = parser.parse_program()?;
    Ok(AST { top: Some(top) })
}

#[cfg(test)]
#[path = "../tests/parser/mod.rs"]
mod parser_tests;
