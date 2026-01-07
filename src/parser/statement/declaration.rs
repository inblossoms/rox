use crate::{
    ast::Stmt,
    parser::{error::Error, parse::ParseHelper},
    tokenizer::TokenType,
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
        let name_token = self.consume(TokenType::Identifier, "Expect function name.")?;
        let name = name_token.clone();

        self.consume(TokenType::LeftParen, "Expect '(' after function name.")?;
        let mut params = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                let arg_token = self.consume(TokenType::Identifier, "Expect parameter name.")?;
                params.push(arg_token.clone());
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(TokenType::LeftBrace, "Expect '{' before function body.")?;
        let previous_func_depth = self.func_depth;
        let previous_loop_depth = self.loop_depth;
        self.func_depth += 1;
        // 进入函数体重置循环深度 因为函数体局部作用域隔离了外部循环
        // while (true) {
        //     fun test() {
        //         break; // break 不应出现在函数体中
        //     }
        // }
        self.loop_depth = 0;
        let body_stmts_result = self.parse_block();
        self.func_depth = previous_func_depth;
        self.loop_depth = previous_loop_depth;

        let body_stmts = body_stmts_result?;

        Ok(Stmt::Function {
            name,
            params,
            body: body_stmts,
        })
    }
}
