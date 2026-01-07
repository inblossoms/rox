use std::collections::HashMap;

use crate::{
    ast::{Expr, ExprId, Stmt},
    evaluate::Interpreter,
    resolver::{FunctionType, LoopType, Resolver},
    tokenizer::Token,
};

impl<'a> Resolver<'a> {
    /// 创建一个新的 Resolver 实例
    ///
    /// # 参数
    /// * `interpreter` - 解释器的可变引用，用于存储解析结果（side table）
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
            current_loop: LoopType::None,
        }
    }

    // 入口

    /// 解析一组语句
    ///
    /// Resolver 的主入口，用于解析整个程序或代码块的 body。
    pub fn resolve_stmts(&mut self, statements: &Vec<Stmt>) -> Result<(), String> {
        for stmt in statements {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }

    // 语句解析

    /// 解析单个语句
    ///
    /// 处理作用域的创建/销毁、变量声明以及控制流的递归解析。
    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            // Block
            // 进入块时创建新作用域，退出时销毁（词法作用域的基础）
            Stmt::Block { body } => {
                self.begin_scope();
                self.resolve_stmts(body)?;
                self.end_scope();
            }

            // 变量声明
            // 处理步骤：声明 (Declare) -> 解析初始化表达式 -> 定义 (Define)。
            // 分步是为了处理 `var a = a;` 自引用错误情况。
            Stmt::VarDecl { name, initializer } => {
                self.declare(name)?;
                if let Some(init) = initializer {
                    self.resolve_expr(init)?;
                }
                self.define(name);
            }

            // 函数声明
            // 函数名在当前作用域立即可见（支持递归），然后创建新作用域解析函数体。
            Stmt::Function { name, params, body } => {
                self.declare(name)?;
                self.define(name);

                self.resolve_function(params, body, FunctionType::Function)?;
            }

            Stmt::Class { name, methods } => {
                self.declare(name)?;
                self.define(name);

                // 当前实现阶段并不运行方法，但也要解析方法体内的变量
                // 可以防止 "return" 出现在方法外等错误
                // TODO: 这里将来要设置 current_function = Method
                for method in methods {
                    if let Stmt::Function {
                        name: _,
                        params,
                        body,
                    } = method
                    {
                        self.resolve_function(params, body, FunctionType::Function)?;
                    }
                }
            }

            // 表达式语句 递归解析内部表达式。
            Stmt::Expression { expr } => {
                self.resolve_expr(expr)?;
            }

            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(then_branch)?;
                if let Some(else_branch) = else_branch {
                    self.resolve_stmt(else_branch)?;
                }
            }

            // 解析循环体时需要更新 `current_loop` 状态，以便检查 break/continue。
            Stmt::While { condition, body } => {
                self.resolve_expr(condition)?;

                let enclosing_loop = self.current_loop;
                self.current_loop = LoopType::Loop;
                self.resolve_stmt(body)?;
                self.current_loop = enclosing_loop;
            }

            // For 循环自带隐式作用域（用于初始化变量），因此显式调用 begin_scope。
            Stmt::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                self.begin_scope();

                if let Some(init) = initializer {
                    self.resolve_stmt(init)?;
                }
                if let Some(cond) = condition {
                    self.resolve_expr(cond)?;
                }
                if let Some(incr) = increment {
                    self.resolve_expr(incr)?;
                }

                let enclosing_loop = self.current_loop;
                self.current_loop = LoopType::Loop;
                self.resolve_stmt(body)?;
                self.current_loop = enclosing_loop;

                self.end_scope();
            }

            Stmt::Print { expr } => {
                self.resolve_expr(expr)?;
            }

            // Return 检查 `return` 是否非法出现在顶层代码中。
            Stmt::Return { keyword, value } => {
                if self.current_function == FunctionType::None {
                    return Err(format!(
                        "[line {}] Can't return from top-level code.",
                        keyword.line
                    ));
                }
                if let Some(val) = value {
                    self.resolve_expr(val)?;
                }
            }

            // Break/Continue 检查是否非法出现在循环外部
            Stmt::Break => {
                if self.current_loop == LoopType::None {
                    return Err("Can't use 'break' outside of a loop.".to_string());
                }
            }
            Stmt::Continue => {
                if self.current_loop == LoopType::None {
                    return Err("Can't use 'continue' outside of a loop.".to_string());
                }
            }

            // 空语句 无需操作
            Stmt::Empty => (),
        }
        Ok(())
    }

    // 表达式解析

    /// 解析单个表达式
    ///
    /// 核心任务是找到所有的 Variable 和 Assign 节点，并调用 `resolve_local`。
    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            // 1. 变量读取 (Variable Access)
            // 这是 Resolver 最核心的逻辑点。
            Expr::Variable { id, name } => {
                // 检查：禁止在初始化器中读取自己 "var a = a;"
                // 此时 `a` 已声明 (in map) 但状态为 false (未定义)。
                if !self.scopes.is_empty() {
                    let scope = self.scopes.last().unwrap();
                    if let Some(false) = scope.get(&name.lexeme) {
                        return Err(format!(
                            "[line {}] Can't read local variable '{}' in its own initializer.",
                            name.line, name.lexeme
                        ));
                    }
                }
                self.resolve_local(id, name);
            }

            // 2. 变量赋值 (Assignment)
            // 先解析右值（确保右值里的变量被解析），再解析左值变量的位置。
            Expr::Assign { id, name, expr } => {
                self.resolve_expr(expr)?;
                self.resolve_local(id, name);
            }

            // 3. 复合赋值 (AssignOp)
            // 逻辑同上。
            Expr::AssignOp { id, name, expr, .. } => {
                self.resolve_expr(expr)?;
                self.resolve_local(id, name);
            }

            // 4. 二元/逻辑运算
            // 递归解析左右子树。
            Expr::Binary { left, right, .. } | Expr::Logical { left, right, .. } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }

            // 5. 一元/分组
            Expr::Unary { expr, .. } | Expr::Grouping { expr } => {
                self.resolve_expr(expr)?;
            }

            // 6. 函数调用 (Call)
            // 递归解析被调用者 (callee) 和参数。
            Expr::Call {
                id: _,
                callee,
                args,
            } => {
                self.resolve_expr(callee)?;

                for arg in args {
                    self.resolve_expr(arg)?;
                }
            }

            // 7. 集合 (递归解析元素)
            Expr::List { elements } | Expr::Tuple { elements } => {
                for e in elements {
                    self.resolve_expr(e)?;
                }
            }
            Expr::Dict { elements } => {
                for (k, v) in elements {
                    self.resolve_expr(k)?;
                    self.resolve_expr(v)?;
                }
            }

            // 8. 字面量 (无需处理)
            Expr::Number { .. } | Expr::String { .. } | Expr::Boolean { .. } | Expr::Nil => {}
        }
        Ok(())
    }

    // --- 核心辅助方法 ---

    /// 开启新作用域
    ///
    /// 向作用域栈压入一个新的 HashMap。
    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// 关闭作用域
    ///
    /// 从作用域栈弹出一个 HashMap，销毁其中定义的局部变量。
    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    /// 声明变量 (Declare)
    ///
    /// 将变量名加入当前作用域，标记为 `false` (未初始化)。
    /// 如果变量名已存在，则报错（禁止在同一作用域重复声明）。
    fn declare(&mut self, name: &Token) -> Result<(), String> {
        if self.scopes.is_empty() {
            return Ok(());
        }

        let scope = self.scopes.last_mut().unwrap();
        if scope.contains_key(&name.lexeme) {
            return Err(format!(
                "[line {}] Already a variable with this name in this scope.",
                name.line
            ));
        }

        scope.insert(name.lexeme.clone(), false);
        Ok(())
    }

    /// 定义变量 (Define)
    ///
    /// 将变量状态更新为 `true` (已初始化/可用)。
    /// 此时变量可以被安全读取。
    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.lexeme.clone(), true);
    }

    /// 核心：解析局部变量 (Resolve Local)
    ///
    /// 从当前作用域开始，向外层作用域遍历，寻找变量声明。
    ///
    /// * 如果找到：计算**距离 (Distance/Hops)**，即当前作用域到声明作用域的层数，
    ///   并将 `(ExprId, distance)` 存入 Interpreter 的侧表。
    /// * 如果没找到：假设它是**全局变量**，不进行任何操作（Interpreter 默认会去全局环境查找）。
    fn resolve_local(&mut self, id: &ExprId, name: &Token) {
        // 从最内层 (scopes.len() - 1) 向外层 (0) 遍历
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                // 找到了！i 就是 distance (跳跃步数)
                self.interpreter.resolve(*id, i);
                return;
            }
        }
    }

    /// 解析函数体
    ///
    /// 创建新作用域，定义参数，然后解析函数体。
    /// 同时负责维护 `current_function` 状态，以便检查 `return`。
    fn resolve_function(
        &mut self,
        params: &Vec<Token>,
        body: &Vec<Stmt>,
        f_type: FunctionType,
    ) -> Result<(), String> {
        let enclosing_func = self.current_function;
        self.current_function = f_type;

        self.begin_scope();

        for param in params {
            self.declare(param)?;
            self.define(param);
        }

        self.resolve_stmts(body)?;

        self.end_scope();

        self.current_function = enclosing_func;
        Ok(())
    }
}
