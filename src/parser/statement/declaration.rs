use crate::{
    ast::{Expr, Stmt},
    parser::{error::Error, parse::ParseHelper},
    tokenizer::{Token, TokenType},
};

// 声明语句（变量声明、函数声明）
impl ParseHelper {
    /// 解析变量声明语句
    pub fn parse_var_declaration(&mut self) -> Result<Stmt, Error> {
        let name_token = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let name = name_token.clone();

        let initializer = if self.match_token(&[TokenType::Equal]) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::VarDecl { name, initializer })
    }

    /// 解析函数声明语句
    pub fn parse_function_declaration(&mut self) -> Result<Stmt, Error> {
        self.parse_function("function")
    }

    fn parse_function(&mut self, kind: &str) -> Result<Stmt, Error> {
        let name = self
            .consume(TokenType::Identifier, &format!("Expect {} name.", kind))?
            .clone();

        let (params, body) = self.parse_function_params_and_body(kind)?;

        Ok(Stmt::Function { name, params, body })
    }

    pub fn parse_lambda(&mut self) -> Result<Expr, Error> {
        // !! 'fun' 在 parse_primary 中被 match 消耗了
        let (params, body) = self.parse_function_params_and_body("lambda")?;

        Ok(Expr::Lambda {
            id: self.generate_id(),
            params,
            body,
        })
    }

    pub fn parse_class_declaration(&mut self) -> Result<Stmt, Error> {
        let name = self
            .consume(TokenType::Identifier, "Expect class name.")?
            .clone();

        let superclass = if self.match_token(&[TokenType::Less]) {
            self.consume(TokenType::Identifier, "Expect superclass name.")?;
            // 父类必须是一个变量引用
            Some(Expr::Variable {
                id: self.generate_id(),
                name: self.previous().clone(),
            })
        } else {
            None
        };

        self.consume(TokenType::LeftBrace, "Expect '{' before class body.")?;

        let mut methods = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.parse_function("method")?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after class body.")?;

        Ok(Stmt::Class {
            name,
            superclass,
            methods,
        })
    }

    /// 辅助方法：解析函数的参数列表和函数体
    ///
    /// # 参数
    /// * `kind` - 函数类型描述（如 "function" 或 "lambda"），用于生成错误信息
    ///
    /// # 返回值
    /// * `Ok((Vec<Token>, Vec<Stmt>))` - 返回解析出的 (参数列表, 函数体语句)
    fn parse_function_params_and_body(
        &mut self,
        kind: &str,
    ) -> Result<(Vec<Token>, Vec<Stmt>), Error> {
        // 1. 解析参数列表
        // 注意：这里的报错信息可以稍微泛化，或者根据 kind 格式化
        self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {} declaration.", kind),
        )?;

        let mut params = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if params.len() >= 255 {
                    let _ = self.error(self.peek(), "Can't have more than 255 parameters.");
                }

                params.push(
                    self.consume(TokenType::Identifier, "Expect parameter name.")?
                        .clone(),
                );

                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        // 2. 解析函数体前的左花括号
        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' before {} body.", kind),
        )?;

        // 3. 上下文维护 (核心复用逻辑)
        let previous_func_depth = self.func_depth;
        let previous_loop_depth = self.loop_depth;

        self.func_depth += 1;
        self.loop_depth = 0; // 函数体隔离外部循环

        // 4. 解析块
        let body_result = self.parse_block();

        // 5. 恢复上下文
        self.func_depth = previous_func_depth;
        self.loop_depth = previous_loop_depth;

        let body = body_result?;

        Ok((params, body))
    }

    pub fn parse_export_statement(&mut self) -> Result<Stmt, Error> {
        // export 后面只能跟声明语句 (var, fun, class)

        let stmt = if self.match_token(&[TokenType::Class]) {
            self.parse_class_declaration()?
        } else if self.match_token(&[TokenType::Fun]) {
            self.parse_function_declaration()?
        } else if self.match_token(&[TokenType::Var]) {
            self.parse_var_declaration()?
        } else {
            return Err(self.error(self.peek(), "Expect declaration after 'export'."));
        };

        Ok(Stmt::Export {
            stmt: Box::new(stmt),
        })
    }
}
