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

    /// 检查是否到达输入流末尾
    ///
    /// # 返回值
    /// * `bool` - 如果到达末尾返回 true，否则返回 false
    pub fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    /// Current Token
    ///
    /// 获取当前未处理的 token
    ///
    /// # 返回值
    /// * `&Token` - 当前 token 的引用
    pub fn peek(&self) -> &Token {
        if self.index >= self.tokens.tokens.len() {
            // 返回最后一个 token (通常是 EOF)，防止越界 panic
            return &self.tokens.tokens[self.tokens.tokens.len() - 1];
        }
        &self.tokens.tokens[self.index]
    }

    /// Previous Token
    ///
    /// 获取上一个已处理的 token
    ///
    /// # 返回值
    /// * `&Token` - 上一个 token 的引用
    pub fn previous(&self) -> &Token {
        if self.index == 0 {
            return &self.tokens.tokens[0];
        }
        &self.tokens.tokens[self.index - 1]
    }

    /// 检查当前 Token 类型是否匹配
    ///
    /// # 参数
    /// * `token_type` - 要检查的 token 类型
    ///
    /// # 返回值
    /// * `bool` - 如果匹配返回 true，否则返回 false
    pub fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() && token_type != TokenType::Eof {
            return false;
        }
        self.peek().token_type == token_type
    }

    /// 如果当前 Token 匹配指定类型，则消耗并返回 true
    ///
    /// # 参数
    /// * `types` - 要匹配的 token 类型列表
    ///
    /// # 返回值
    /// * `bool` - 如果匹配并消耗了 token 返回 true，否则返回 false
    pub fn match_token(&mut self, types: &[TokenType]) -> bool {
        for &t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// 返回当前 Token
    ///
    /// # 返回值
    /// * `&Token` - token 引用
    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.index += 1;
        }
        self.previous()
    }

    /// 强制要求当前 Token 为指定类型，否则返回错误
    ///
    /// # 参数
    /// * `token_type` - 期望的 token 类型
    /// * `message` - 错误消息
    pub fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, Error> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(self.error(self.peek(), message))
        }
    }

    /// 创建错误实例
    ///
    /// # 参数
    /// * `token` - 错误相关的 token
    /// * `message` - 错误消息
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
        // 优先级：乘除 > 加减 > 位与 > 位或 > 比较 > 相等 > 逻辑与 > 逻辑或 > 赋值
        let expr = self.parse_or()?;

        if self.match_token(&[
            TokenType::Equal,
            TokenType::PlusEqual,
            TokenType::MinusEqual,
            TokenType::StarEqual,
            TokenType::SlashEqual,
        ]) {
            // 先保存操作符 Token，parse_assignment() 会消耗新的 Token，
            // 导致 self.previous() 变成右值表达式的最后一个 Token，而不是操作符。
            let operator_token = self.previous().clone();

            let value = self.parse_assignment()?;

            // 检查左值是否合法
            if let Expr::Variable { name } = expr {
                // 使用保存的 operator_token 进行匹配
                match operator_token.token_type {
                    TokenType::Equal => {
                        return Ok(Expr::Assign {
                            name,
                            expr: Box::new(value),
                        });
                    }
                    TokenType::PlusEqual => {
                        return Ok(Expr::AssignOp {
                            op: Operator::Add,
                            name,
                            expr: Box::new(value),
                        });
                    }
                    TokenType::MinusEqual => {
                        return Ok(Expr::AssignOp {
                            op: Operator::Sub,
                            name,
                            expr: Box::new(value),
                        });
                    }
                    TokenType::StarEqual => {
                        return Ok(Expr::AssignOp {
                            op: Operator::Mul,
                            name,
                            expr: Box::new(value),
                        });
                    }
                    TokenType::SlashEqual => {
                        return Ok(Expr::AssignOp {
                            op: Operator::Div,
                            name,
                            expr: Box::new(value),
                        });
                    }
                    _ => {}
                }
            }

            // 报错时使用 operator_token 定位，指向操作符位置更准确
            return Err(self.error(&operator_token, "Invalid assignment target."));
        }

        Ok(expr)
    }

    /// OR
    pub fn parse_or(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_and()?;

        while self.match_token(&[TokenType::Or]) {
            let op = Operator::LogicalOr;
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
            let op = Operator::LogicalAnd;
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

    /// 语句解析入口
    /// 支持的语句类型：
    /// - if 语句
    /// - while 语句  
    /// - var 变量声明
    /// - fun 函数声明
    /// - 代码块 { ... }
    /// - 表达式语句（默认分支）
    ///
    /// # 返回值
    /// * `Ok(Expr)` - 解析得到的表达式
    /// * `Err(Error)` - 解析过程中发生的错误
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
        if self.match_token(&[TokenType::For]) {
            return self.parse_for_statement();
        }
        if self.match_token(&[TokenType::Break]) {
            return self.parse_break_statement();
        }
        if self.match_token(&[TokenType::Continue]) {
            return self.parse_continue_statement();
        }
        if self.match_token(&[TokenType::Print]) {
            return self.parse_print_statement();
        }
        // TODO: 添加 Class、super、this 逻辑实现

        // 解析表达式语句（以分号结尾的表达式）
        self.parse_expression_statement()
    }

    /// 解析表达式语句，并返回表达式 AST 节点（树）
    pub fn parse_expression_statement(&mut self) -> Result<Expr, Error> {
        let expr = self.parse_expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(expr)
    }

    /// 程序解析入口
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
    /// 创建解析器实例
    ///
    /// # 参数
    /// * `tokens` - 词法分析器输出的 token 列表
    ///
    /// # 返回值
    /// * `Parser` - 新的解析器实例
    pub fn new(tokens: Tokens) -> Self {
        Self {
            helper: ParseHelper { tokens, index: 0 },
        }
    }

    /// 解析器入口
    ///
    /// # 返回值
    /// * `Result<Expr, Error>` - 解析得到的表达式或错误
    pub fn parse_program(&mut self) -> Result<Expr, Error> {
        self.helper.parse_program()
    }
}

/// 解析器
///
/// # 参数
/// * `tokens` - 词法分析器输出的 token 列表
///
/// # 返回值
/// * `Result<AST, Error>` - 解析得到的抽象语法树或错误
pub fn parse(tokens: Tokens) -> Result<AST, Error> {
    let mut parser = Parser::new(tokens);
    let top = parser.parse_program()?;
    Ok(AST { top: Some(top) })
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod parser_tests;
