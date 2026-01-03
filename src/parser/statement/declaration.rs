use crate::{
    ast::Expr,
    parser::{error::Error, parse::ParseHelper},
    tokenizer::TokenType,
};

// 声明语句（变量声明、函数声明）
impl ParseHelper {
    /// 解析变量声明语句
    pub fn parse_var_declaration(&mut self) -> Result<Expr, Error> {
        let name_token = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let name = name_token.lexeme.clone();

        let initializer = if self.match_token(&[TokenType::Equal]) {
            self.parse_expression()?
        } else {
            Expr::Nil
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Expr::VarDecl {
            name,
            initializer: Box::new(initializer),
        })
    }

    /// 解析函数声明语句
    pub fn parse_function_declaration(&mut self) -> Result<Expr, Error> {
        let name_token = self.consume(TokenType::Identifier, "Expect function name.")?;
        let name = name_token.lexeme.clone();

        self.consume(TokenType::LeftParen, "Expect '(' after function name.")?;
        let mut args = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                let arg_token = self.consume(TokenType::Identifier, "Expect parameter name.")?;
                args.push(arg_token.lexeme.clone());
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(TokenType::LeftBrace, "Expect '{' before function body.")?;

        let previous_loop_depth = self.loop_depth;
        // 进入函数体重置循环深度 因为函数体局部作用域隔离了外部循环
        // while (true) {
        //     fun test() {
        //         break; // 语法错误！虽然行为上被包含在 while 的花括号里，但逻辑上在函数里
        //     }
        // }
        self.loop_depth = 0;
        let body_stmts_result = self.parse_block();
        self.loop_depth = previous_loop_depth;

        let body_stmts = body_stmts_result?;

        Ok(Expr::Function {
            name,
            args,
            body: body_stmts,
        })
    }
}
