use crate::{
    ast::{Expr, ExprId},
    parser::{error::Error, parse::ParseHelper},
    tokenizer::{Literal, TokenType},
};

impl ParseHelper {
    /// 解析函数调用表达式
    ///
    /// 语法规则: `call -> primary ( "(" arguments? ")" )*`
    ///
    /// 该方法首先解析一个基本表达式（通常是函数名，但也可能是返回函数的表达式）。
    /// 然后进入循环，不断检查后续是否有 `(`，以支持链式调用。
    /// 例如：`getCallback()(arg)` 会被解析为两次调用。
    ///
    /// # 返回值
    /// * `Result<Expr, Error>` - 解析完成的 Call 表达式或错误
    pub fn parse_call(&mut self) -> Result<Expr, Error> {
        // 解析左侧的 "被调用者" (Callee)
        let mut expr = self.parse_primary()?;

        // 循环检查是否有参数列表，处理 func(a)(b) 这种情况
        loop {
            if self.match_token(&[TokenType::LeftParen]) {
                // 发现左括号，说明是函数调用，递归解析参数并包装 expr
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    /// 完成函数调用的解析
    ///
    /// 负责解析 `(` 之后的参数列表以及闭合的 `)`。
    ///
    /// # 逻辑流程
    /// 1. 检查是否直接遇到 `)` (无参调用)。
    /// 2. 如果有参数，循环解析表达式，并处理逗号分隔符。
    /// 3. 检查参数数量是否超过 255 (但只报错不停止)。
    /// 4. 消耗右括号 `)`。
    /// 5. 将传入的 `callee` 和解析出的 `args` 包装成一个新的 `Expr::Call` 节点。
    ///
    /// # 参数
    /// * `callee` - 被调用的表达式（即左括号左边的部分）
    pub fn finish_call(&mut self, callee: Expr) -> Result<Expr, Error> {
        let mut args = Vec::new();

        // 如果不是立即遇到右括号，说明有参数
        if !self.check(TokenType::RightParen) {
            loop {
                // 限制参数数量，通常只是一个非强制性限制
                if args.len() >= 255 {
                    // 报告错误但不中断解析
                    let _ = self.error(self.peek(), "Can't have more than 255 arguments.");
                }

                // 解析参数表达式
                args.push(self.parse_expression()?);

                // 如果没有逗号，说明参数列表结束
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;

        // 构造 Call 节点
        // Note：callee 被装箱 (Box)，args 被移动
        Ok(Expr::Call {
            id: self.generate_id(), // 为这次调用分配唯一 ID (用于 Resolver)
            callee: Box::new(callee),
            args,
        })
    }

    /// 解析基本表达式
    ///
    /// 表达式优先级的最高层（也是递归下降的最底层）。
    /// 它处理语言中的原子元素：字面量、变量访问和括号分组。
    ///
    /// # 支持的类型
    /// * **字面量**: `true`, `false`, `nil`, 数字, 字符串
    /// * **变量**: 标识符 (Identifier) -> 生成 `Expr::Variable`
    /// * **分组**: `( expression )` -> 生成 `Expr::Grouping`
    ///
    /// # 错误处理
    /// 如果当前 Token 不匹配上述任何情况，将返回 "Unexpected token" 错误。
    pub fn parse_primary(&mut self) -> Result<Expr, Error> {
        // --- 字面量 ---
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
            // 从 Token 的 Literal 中提取值
            let value = match &self.previous().literal {
                Literal::Number(n) => n.to_string(),
                _ => "0".to_string(), // 防御性默认值
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
            // 变量引用需要 ID 供 Resolver 使用
            return Ok(Expr::Variable {
                name: self.previous().clone(),
                id: self.generate_id(),
            });
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.parse_expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
            return Ok(Expr::Grouping {
                expr: Box::new(expr),
            });
        }

        // 错误情况
        let current_token = self.peek();
        Err(self.error(current_token, &format!(
            "Unexpected token '{:?}'. Expected a primary expression (boolean, number, string, identifier, or grouping expression).",
            current_token.token_type
        )))
    }

    /// Generate Unique ExprId
    ///
    /// # 用途
    /// Resolver (语义分析器) 使用此 ID 来区分源代码中不同位置的同一个变量名。
    /// eg. 在 `var a = a;` 中，左边和右边的 `a` 拥有不同的 ID，
    /// Resolver 可以据此判断右边的 `a` 是否引用了未初始化的左边的 `a`。
    pub fn generate_id(&mut self) -> ExprId {
        let id = self.next_id;
        self.next_id += 1;
        ExprId(id)
    }
}
