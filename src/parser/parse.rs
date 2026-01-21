use crate::{
    ast::{AST, Expr, Operator, stmt::Stmt},
    parser::error::Error,
    tokenizer::{Token, TokenType, Tokens},
};

/// 状态机 (Parser State Helper)
///
/// 这个结构体持有解析过程中的所有可变状态。
/// 负责遍历 Token 流，并维护上下文信息（循环深度、函数深度），
/// 以便在解析阶段直接进行部分语法错误检查（Fail Fast）。
#[derive(Debug)]
pub struct ParseHelper {
    // tokens & index 解析器输入流读取机制。
    /// Token 流
    ///
    /// 词法分析器 (Tokenizer) 生成的扁平化 Token 列表。
    /// 解析器通过 `index` 在这个列表中移动。
    pub tokens: Tokens,

    /// 当前指针位置
    ///
    /// 指向 `tokens` 列表中当前正在处理的 Token 索引。
    /// - `peek()` 读取 `tokens[index]`
    /// - `advance()` 会导致 `index += 1`
    pub index: usize,

    // loop_depth & func_depth 上下文感知 (Context Awareness) 机制，允许解析器在生成 AST 之前就拦截结构性错误（如在循环外 break）。
    /// Loop Depth
    ///
    /// 记录当前代码处于多少层循环 (`for`, `while`) 内部。
    /// - 进入循环时 +1，退出时 -1。
    /// - **用途**：检查 `break` 和 `continue` 语句的合法性。
    ///   如果 `loop_depth == 0`，说明这两个语句出现在循环之外，应报错。
    pub loop_depth: usize,

    /// Function Depth
    ///
    /// 记录当前代码处于多少层函数声明内部。
    /// - 进入 `fun` 定义时 +1，退出时 -1。
    /// - **用途**：检查 `return` 语句的合法性。
    ///   如果 `func_depth == 0` (顶层代码)，使用 `return` 应报错。
    pub func_depth: usize,

    // next_id: 静态分析，为 Resolver 和 Interpreter 提供正确处理闭包和变量遮蔽的条件。
    /// Unique ID Generator 计数器
    ///
    /// 用于生成全局唯一的 `ExprId`。
    /// - 每次创建一个涉及变量引用 (`Variable`, `Assign`, `Call`) 的 AST 节点时，
    ///   该计数器自增并分配 ID。
    /// - **用途**：**Resolver** (语义分析器) 需要利用这个 ID 来区分源代码中
    ///   不同的变量访问位置，以便在 `Interpreter` Side Table (`locals`) 中存储
    ///   对应的作用域跳跃距离 (Distance)。
    pub next_id: usize,
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

    //  (Helper Methods)

    /// 查看当前 Token
    ///
    /// 返回当前指针指向的 Token，但**不消耗**（不移动指针位置）。
    /// 当指针越界时，返回最后一个 Token (通常是 EOF)，防止数组越界 Panic。
    ///
    /// # 返回值
    /// * `&Token` - 当前 Token 的引用
    pub fn peek(&self) -> &Token {
        if self.index >= self.tokens.tokens.len() {
            // 返回最后一个 token (通常是 EOF)，防止越界 panic
            return &self.tokens.tokens[self.tokens.tokens.len() - 1];
        }
        &self.tokens.tokens[self.index]
    }

    /// 获取上一个 Token
    ///
    /// 返回指针上一个位置的 Token。通常用于 `advance()` 或 `match_token()` 作用域中。
    /// 消耗了一个 Token 之后，获取那个刚刚被消耗的 Token 的信息（如获取变量名、操作符类型等）。
    ///
    /// # 返回值
    /// * `&Token` - 上一个 Token 的引用
    pub fn previous(&self) -> &Token {
        if self.index == 0 {
            return &self.tokens.tokens[0];
        }
        &self.tokens.tokens[self.index - 1]
    }

    /// 检查当前 Token 类型 (Check Type)
    ///
    /// 查看当前的 Token 是否属于指定的类型，但**不消耗**。
    /// 如果当前已经到达 EOF，除非检查的是 EOF 类型，否则返回 false。
    ///
    /// # 参数
    /// * `token_type` - 期望的 Token 类型
    ///
    /// # 返回值
    /// * `bool` - 如果类型匹配返回 true，否则返回 false
    pub fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() && token_type != TokenType::Eof {
            return false;
        }
        self.peek().token_type == token_type
    }

    /// 匹配并消耗 Token
    ///
    /// 检查当前 Token 是否属于给定的类型列表中的任意一种。
    /// - 如果匹配：**消耗**该 Token（游标前移），并返回 `true`。
    /// - 如果不匹配：什么都不做（游标不动），并返回 `false`。
    ///
    /// 在递归下降分析中处理可选语法分支（eg. 二元运算、控制流关键字）的主要方法。
    ///
    /// # 参数
    /// * `types` - 可能的 Token 类型列表
    ///
    /// # 返回值
    /// * `bool` - 是否成功匹配并消耗了 Token
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
    ///
    /// 将 Parser 的指针位置向后移动一位（消耗当前 Token），并返回**移动前**的那个 Token。
    ///
    /// # 返回值
    /// * `&Token` - 被消耗的那个 Token 的引用
    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.index += 1;
        }
        self.previous()
    }

    /// 强制消耗 Token
    ///
    /// 验证当前 Token 是否为期望的类型：
    /// - 如果是：消耗它并返回该 Token。
    /// - 如果不是：**报错**。这用于处理语法中必须出现的部分（如右括号、分号）。
    ///
    /// # 参数
    /// * `token_type` - 必须匹配的 Token 类型
    /// * `message` - 匹配失败时报错的提示信息
    ///
    /// # 返回值
    /// * `Ok(&Token)` - 成功消耗的 Token
    /// * `Err(Error)` - 语法错误
    pub fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, Error> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(self.error(self.peek(), message))
        }
    }

    /// 构造语法错误
    ///
    /// 创建一个包含行号和错误信息的 `Error` 对象。
    /// 不期望导致 Panic，而是返回一个 Result::Err 供上层处理或同步恢复。
    ///
    /// # 参数
    /// * `token` - 触发错误的 Token（用于定位行号）
    /// * `message` - 具体的错误描述
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

            let value = self.parse_assignment()?; // 递归解析右值

            match expr {
                // 检查左值是否合法
                Expr::Variable { name, .. } => {
                    let id = self.generate_id();
                    // 使用保存的 operator_token 进行匹配
                    match operator_token.token_type {
                        TokenType::Equal => Ok(Expr::Assign {
                            id,
                            name,
                            expr: Box::new(value),
                        }),
                        TokenType::PlusEqual => Ok(Expr::AssignOp {
                            id,
                            name,
                            op: Operator::Add,
                            expr: Box::new(value),
                        }),
                        TokenType::MinusEqual => Ok(Expr::AssignOp {
                            id,
                            name,
                            op: Operator::Sub,
                            expr: Box::new(value),
                        }),
                        TokenType::StarEqual => Ok(Expr::AssignOp {
                            id,
                            name,
                            op: Operator::Mul,
                            expr: Box::new(value),
                        }),
                        TokenType::SlashEqual => Ok(Expr::AssignOp {
                            id,
                            name,
                            op: Operator::Div,
                            expr: Box::new(value),
                        }),
                        _ => unreachable!(),
                    }
                }

                // 对象属性赋值 (Set：赋值行为)
                // 如果左值是一个 Get 表达式 (a.b)，转换为 Set 表达式 (a.b = value)
                Expr::Get { object, name } => {
                    match operator_token.token_type {
                        TokenType::Equal => Ok(Expr::Set {
                            object,
                            name,
                            value: Box::new(value),
                        }),
                        // TODO: 支持 a.b += 1，扩展 Expr::SetOp 或者类似的逻辑
                        // 暂时只处理 =
                        _ => Err(self.error(
                            &operator_token,
                            "Compound assignment not supported on properties yet.",
                        )),
                    }
                }
                Expr::GetIndex {
                    object,
                    index,
                    bracket,
                    ..
                } => {
                    match operator_token.token_type {
                        TokenType::Equal => {
                            // 转换为 SetIndex
                            Ok(Expr::SetIndex {
                                id: self.generate_id(),
                                object,
                                index,
                                bracket,
                                value: Box::new(value),
                            })
                        }

                        // TODO: 支持 arr[i] += 1
                        _ => Err(self.error(
                            &operator_token,
                            "Compound assignment not supported on subscripts yet.",
                        )),
                    }
                }

                // 报错时使用 operator_token 定位，指向操作符位置更准确
                // 对象属性赋值 (Set)
                // 如果左值是一个 Get 表达式 (例如 a.b)，将其转换为 Set 表达式 (a.b = value)
                // 报错时使用 operator_token 定位，指向操作符位置更准确
                _ => Err(self.error(&operator_token, "Invalid assignment target.")),
            }
        } else {
            Ok(expr)
        }
    }

    /// OR
    pub fn parse_or(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_and()?;

        while self.match_token(&[TokenType::LogicalOr, TokenType::Or]) {
            let op = Operator::LogicalOr;
            let right = self.parse_and()?;
            expr = Expr::Logical {
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

        // and 和 && 采用相同的逻辑
        while self.match_token(&[TokenType::LogicalAnd, TokenType::And]) {
            let op = Operator::LogicalAnd;
            let right = self.parse_equality()?;
            expr = Expr::Logical {
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
    pub fn parse_statement(&mut self) -> Result<Stmt, Error> {
        if self.match_token(&[TokenType::If]) {
            return self.parse_if_statement();
        }
        if self.match_token(&[TokenType::Try]) {
            return self.parse_try_statement();
        }
        if self.match_token(&[TokenType::Throw]) {
            return self.parse_throw_statement();
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
        if self.match_token(&[TokenType::Class]) {
            return self.parse_class_declaration();
        }
        // 允许空语句: ";", "for(;;);", "{ ; }"
        if self.match_token(&[TokenType::Semicolon]) {
            return Ok(Stmt::Empty);
        }
        if self.match_token(&[TokenType::LeftBrace]) {
            let statements = self.parse_block()?;
            return Ok(Stmt::Block { body: statements });
        }
        if self.match_token(&[TokenType::Return]) {
            return self.parse_return_statement();
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
        if self.match_token(&[TokenType::Export]) {
            return self.parse_export_statement();
        }
        // 解析表达式语句（以分号结尾的表达式）
        self.parse_expression_statement()
    }

    /// 语法: "[" ( expression ( "," expression )* )? "]"
    pub fn parse_list(&mut self) -> Result<Expr, Error> {
        let mut elements = Vec::new();

        // 如果不是空列表
        if !self.check(TokenType::LeftBracket) {
            loop {
                // 解析元素
                elements.push(self.parse_expression()?);

                // 如果没有逗号，停止循环
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightBracket, "Expect ']' after list elements.")?;

        Ok(Expr::List { elements })
    }

    /// 语法: "{" ( key ":" value ( "," key ":" value )* )? "}"
    pub fn parse_dict(&mut self) -> Result<Expr, Error> {
        let mut elements = Vec::new();

        // 如果不是空字典
        if !self.check(TokenType::LeftBrace) {
            loop {
                let key = self.parse_expression()?;

                self.consume(TokenType::Colon, "Expect ':' after dictionary key.")?;

                let value = self.parse_expression()?;

                elements.push((key, value));

                // 如果没有逗号，停止循环
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(
            TokenType::RightBrace,
            "Expect '}' after dictionary elements.",
        )?;

        Ok(Expr::Dict { elements })
    }

    /// 解析表达式语句 (Expression Statement)
    ///
    /// 对应语法规则: `exprStmt -> expression ";"`
    ///
    /// 这是一个产生副作用的语句（eg. `print();` 或赋值 `a=1;`）。
    /// 它解析一个表达式，并强制要求一个分号结尾，并将其包装为 `Stmt::Expression`。
    pub fn parse_expression_statement(&mut self) -> Result<Stmt, Error> {
        // 表达式解析(提取 Expr)
        let expr = self.parse_expression()?;

        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;

        // 包装 Expr 进 Stmt::Expression
        Ok(Stmt::Expression { expr })
    }
    /// 程序解析入口
    pub fn parse_program(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
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
            helper: ParseHelper {
                tokens,
                index: 0,
                loop_depth: 0,
                func_depth: 0,
                next_id: 0,
            },
        }
    }

    /// 解析器入口
    ///
    /// # 返回值
    /// * `Result<Expr, Error>` - 解析得到的表达式或错误
    pub fn parse_program(&mut self) -> Result<Vec<Stmt>, Error> {
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
    let statements = parser.parse_program()?;
    Ok(AST { body: statements })
}
