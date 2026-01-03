use crate::{
    ast::Expr,
    parser::{error::Error, parse::ParseHelper},
    tokenizer::TokenType,
};

// 处理控制语句行为
impl ParseHelper {
    /// 解析 if 语句
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

    /// 解析 while 语句
    pub fn parse_while_statement(&mut self) -> Result<Expr, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;

        self.loop_depth += 1; // 进入循环

        // 解析 body 的过程中可能存在递归调用 parse_statement -> parse_break
        // 所以这里不着急 ? 返回，先拿到结果
        let body_result = self.parse_statement();

        // 退出循环，无论 body 是否解析成功，都主动还原深度（即使解析失败）
        self.loop_depth -= 1;

        let body = body_result?; // 在这里处理错误

        Ok(Expr::While {
            condition: Box::new(condition),
            body: Box::new(body),
        })
    }

    /// 解析 for 循环语句
    pub fn parse_for_statement(&mut self) -> Result<Expr, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        // 1. 初始化
        let initializer = if self.match_token(&[TokenType::Semicolon]) {
            None // for (;...)
        } else if self.match_token(&[TokenType::Var]) {
            Some(Box::new(self.parse_var_declaration()?)) // for (var i=0;...)
        } else {
            Some(Box::new(self.parse_expression_statement()?)) // var i; for (i=0;...)
        };
        // Note：parse_var_declaration 和 parse_expression_statement 内部通常已经消耗了分号
        //       对于是 None 的情况，match_token 已经消耗分号，所以这里不需要再处理分号

        // 2. 条件
        let condition = if self.check(TokenType::Semicolon) {
            None // 空条件默认为 true，for (...; ;...)
        } else {
            Some(Box::new(self.parse_expression()?))
        };
        self.consume(TokenType::Semicolon, "Expect ';' after for condition.")?;

        // 3. 增量
        let increment = if self.check(TokenType::RightParen) {
            None // 没有增量，for (...;...; )
        } else {
            Some(Box::new(self.parse_expression()?))
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        // 4. 循环体
        self.loop_depth += 1;
        let body_result = self.parse_statement();
        self.loop_depth -= 1;

        let body = body_result?;
        Ok(Expr::For {
            initializer,
            condition,
            increment,
            body: Box::new(body),
        })
    }

    /// 解析 break 语句
    pub fn parse_break_statement(&mut self) -> Result<Expr, Error> {
        if self.loop_depth == 0 {
            return Err(self.error(self.previous(), "Cannot use 'break' outside of a loop."));
        }

        self.consume(TokenType::Semicolon, "Expect ';' after 'break'.")?;
        Ok(Expr::Break)
    }

    /// 解析 continue 语句
    pub fn parse_continue_statement(&mut self) -> Result<Expr, Error> {
        if self.loop_depth == 0 {
            return Err(self.error(self.previous(), "Cannot use 'continue' outside of a loop."));
        }
        self.consume(TokenType::Semicolon, "Expect ';' after 'break'.")?;
        Ok(Expr::Continue)
    }
}
