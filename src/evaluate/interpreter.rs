use crate::ast::{AST, Expr, Operator};
use crate::evaluate::{
	value::Value,
	environment::Environment
};
use std::{
	cell::RefCell,
	collections::HashMap,
	rc::Rc
};

#[derive(Debug)]
pub struct Error {}

#[derive(Debug)]
#[allow(dead_code)]
pub enum RuntimeError {
    Generic(String),
    UndefinedVariable(String),
    TypeError(String),
    DivisionByZero,
    Return(Value), 
	 Print(String),
    Break,
    Continue,
}

pub struct Interpreter {
    /// 当前执行环境的引用计数指针，用于变量查找和赋值
    pub environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    /// 创建新的解释器实例，初始化全局环境
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    /// 解释执行抽象语法树 (AST)
    /// 
    /// 如果顶层节点是 Block，则直接在当前环境中执行其中的语句
    /// 否则对顶层表达式进行求值
    /// 
    /// # 参数
    /// * `ast` - 要执行的抽象语法树
    /// 
    /// # 返回值
    /// * `Ok(Value)` - 执行结果值
    /// * `Err(RuntimeError)` - 执行过程中发生的错误
    pub fn interpret(&mut self, ast: AST) -> Result<Value, RuntimeError> {
        if let Some(expr) = ast.top {
            // 如果顶层节点是 Block（由 parse_program 生成），
            // 需要“解包”这个 Block，直接在当前环境执行其中的语句，
            // 而不是调用 self.evaluate(&expr) —— 后者会创建一个临时的子作用域。
            if let Expr::Block { body } = expr {
                let mut result = Value::Nil;
                for stmt in body {
                    result = self.evaluate(&stmt)?;
                }
                return Ok(result);
            }

            let result = self.evaluate(&expr);
        
            match result {
                Err(RuntimeError::Break) => Err(RuntimeError::Generic("Cannot use 'break' outside of a loop.".into())),
                Err(RuntimeError::Continue) => Err(RuntimeError::Generic("Cannot use 'continue' outside of a loop.".into())),
                _ => result,
            }
        } else {
            Ok(Value::Nil)
		  }
    }

    /// 解释器的核心，递归地处理 AST 中的表达式节点进行求值，并返回结果
    /// 
    /// # 参数
    /// * `expr` - 要求值的表达式
    /// 
    /// # 返回值
    /// * `Ok(Value)` - 求值结果
    /// * `Err(RuntimeError)` - 求值过程中发生的错误
    fn evaluate(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Break => Err(RuntimeError::Break),
            Expr::Continue => Err(RuntimeError::Continue),
            Expr::Number { value } => {
                let n = value
                    .parse::<f64>()
                    .map_err(|_| RuntimeError::Generic("Invalid number".into()))?;
                Ok(Value::Number(n))
            }
            Expr::String { value } => Ok(Value::String(value.clone())),
            Expr::Boolean { value } => Ok(Value::Boolean(*value)),
            Expr::Nil => Ok(Value::Nil),
            Expr::Grouping { expr } => self.evaluate(expr),
            Expr::Unary { op, expr } => {
                let right = self.evaluate(expr)?;
                match op {
                    Operator::Sub => match right {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => Err(RuntimeError::TypeError("Operand must be a number".into())),
                    },
                    Operator::Not => Ok(Value::Boolean(!right.is_truthy())),
                    _ => Err(RuntimeError::Generic("Invalid unary operator".into())),
                }
            }
            Expr::Binary { left, op, right } => {
                if *op == Operator::LogicalAnd || *op == Operator::LogicalOr {
                    return self.evaluate_logical(left, op, right);
                }

                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;

                match op {
                    Operator::Add => {
                        self.add_values(l, r)
                    }
                    Operator::Sub => {
                        self.check_number_operands(l, r, |a, b| Ok(Value::Number(a - b)))
                    }
                    Operator::Mul => {
                        self.check_number_operands(l, r, |a, b| Ok(Value::Number(a * b)))
                    }
                    Operator::Div => self.check_number_operands(l, r, |a, b| {
                        if b == 0.0 {
                            Err(RuntimeError::DivisionByZero)
                        } else {
                            Ok(Value::Number(a / b))
                        }
                    }),

                    Operator::BitwiseAnd => self.eval_bitwise(left, right, |a, b| a & b),
                    Operator::BitwiseOr  => self.eval_bitwise(left, right, |a, b| a | b),
                    // 添加异或运算符支持
                    Operator::BitwiseXor => self.eval_bitwise(left, right, |a, b| a ^ b),

                    // 比较运算
                    Operator::Greater => {
                        self.check_number_operands(l, r, |a, b| Ok(Value::Boolean(a > b)))
                    }
                    Operator::GreaterEqual => {
                        self.check_number_operands(l, r, |a, b| Ok(Value::Boolean(a >= b)))
                    }
                    Operator::Less => {
                        self.check_number_operands(l, r, |a, b| Ok(Value::Boolean(a < b)))
                    }
                    Operator::LessEqual => {
                        self.check_number_operands(l, r, |a, b| Ok(Value::Boolean(a <= b)))
                    }

                    // 相等运算 (应该支持所有类型)
                    Operator::Equal => Ok(Value::Boolean(l == r)),
                    Operator::NotEqual => Ok(Value::Boolean(l != r)),

                    _ => Err(RuntimeError::Generic("Unknown binary operator".into())),
                }
            }
            Expr::Variable { name } => self
                .environment
                .borrow()
                .get(name)
                .ok_or_else(|| RuntimeError::UndefinedVariable(name.clone())),
            Expr::Assign { name, expr } => {
                let value = self.evaluate(expr)?;
                let success = self.environment.borrow_mut().assign(name, value.clone());
                if success {
                    Ok(value) // 赋值表达式返回赋的值 (e.g. a = b = 2)
                } else {
                    // 使用 assign，递归查找。
                    let success = self.environment.borrow_mut().assign(name, value.clone());
                    
                    if success {
                        Ok(value)
                    } else {
                        // 如果找不到变量应该提供错误信息， 而非自动创建。
                        // 避免隐式全局变量，强制先 var 声明。
                        Err(RuntimeError::UndefinedVariable(name.clone()))
                    }
                }
            }
            Expr::Block { body } => {
                self.execute_block(body, Environment::with_enclosing(self.environment.clone()))
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.evaluate(condition)?.is_truthy() {
                    self.evaluate(then_branch)
                } else if let Some(else_branch) = else_branch {
                    self.evaluate(else_branch)
                } else {
                    Ok(Value::Nil)
                }
            }
            Expr::While { condition, body } => {
                let mut result = Value::Nil;
        
                while self.evaluate(condition)?.is_truthy() {
                    match self.evaluate(body) {
                        Ok(val) => {
                            result = val; // 更新最后一条语句的值
                        }
                        Err(e) => match e {
                            RuntimeError::Break => break,
                            // Note: 借用语言机制 continue 进入下一次循环条件检查
                            RuntimeError::Continue => continue,
                            _ => return Err(e),
                        }
                    }
                }
                Ok(result)
            }
				Expr::For { initializer, condition, increment, body } => {
                // 1. 定义的 i 应该只在循环内部有效 (防止 var i 泄露到外部)
                let previous_env = self.environment.clone();
                self.environment = Rc::new(RefCell::new(Environment::with_enclosing(previous_env.clone())));

                // 2. 初始化 (如果有)
                if let Some(init) = initializer {
                    self.evaluate(init)?;
                }

                let mut result = Value::Nil;

                loop {
                    // 3. 检查条件
                    if let Some(cond) = condition {
                        if !self.evaluate(cond)?.is_truthy() {
                            break; // 条件为假，退出循环
                        }
                    } else {
                        // 如果没有条件 (for(;;))，默认为 true 死循环
                    }

                    // 4. 执行循环体
                    match self.evaluate(body) {
                        Ok(val) => result = val,
                        Err(e) => match e {
                            RuntimeError::Break => break, // 捕获 break，退出 loop
                            RuntimeError::Continue => {
                                // Note：Continue 后必须执行 increment 增量操作，不能直接跳过
                                // 这里什么都不做，让代码流继续走到下面的 increment 执行处
                            }
                            _ => {
                                // 发生错误，需要恢复环境并抛出
                                self.environment = previous_env;
                                return Err(e);
                            }
                        }
                    }

                    // 5. 执行增量操作：无论是正常执行完 body 还是遇到了 continue，都要执行这一步
                    if let Some(incr) = increment {
                        self.evaluate(incr)?;
                    }
                }

                // 6. 恢复环境 (销毁 loop 变量 i)
                self.environment = previous_env;
                Ok(result)
            }
            Expr::Function { name, args, body } => {
                let function = Value::Function {
                    name: name.clone(),
                    args: args.clone(),
                    body: body.clone(),
                    // 捕获当前环境
                    closure: self.environment.clone(),
                };
                // 检查函数是否已存在，避免重复定义
                let env_borrow = self.environment.borrow();
                if env_borrow.get(name).is_some() {
                    drop(env_borrow); // 释放借用
                    return Err(RuntimeError::Generic(format!("Function '{}' is already defined", name)));
                } else {
                    drop(env_borrow); // 释放借用
                }
                // 将函数定义在当前环境中
                self.environment.borrow_mut().define(name.clone(), function);
                Ok(Value::Nil)
            }
				// Note: 对应 AST 中的函数调用表达式：name(arg1, arg2, ...)
            Expr::Call { name, args } => {
                // 1. 获取被调用对象 
                // 根据 AST 定义，Call 存储的是函数名字符串。
                // 我们先将其作为变量进行求值，以获取内存中的 Value::Function 对象。
                let callee = self.evaluate(&Expr::Variable { name: name.clone() })?;

                match callee {
                    Value::Function { 
                        args: param_names, // 形参列表 (定义时的参数名)
                        body,              // 函数体 (AST 节点列表)
                        closure,           // 闭包 (定义时的环境快照)
                        ..                 // 忽略 name 字段
                    } => {
                        // 2. Arity Check
                        if args.len() != param_names.len() {
                            return Err(RuntimeError::Generic(format!(
                                "Expected {} arguments but got {}.",
                                param_names.len(),
                                args.len()
                            )));
                        }

                        // 3. 获取参数值
                        // 注意：参数必须在 *当前环境* (调用者的环境) 下求值
                        let mut arg_values = Vec::new();
                        for arg in args {
                            arg_values.push(self.evaluate(arg)?);
                        }

                        // 4. 创建函数作用域
                        // 关键点：新环境的父环境必须是 **closure** (定义时的环境)，
                        //         而不是 self.environment (调用时的环境)。
                        //         只有这样才能实现词法作用域 (Lexical Scoping)。
                        let func_env = Rc::new(RefCell::new(Environment::with_enclosing(closure)));

                        // 5. 绑定参数 
                        for (i, param_name) in param_names.iter().enumerate() {
                            // 将计算好的实参值，绑定到新环境中的形参名上
                            func_env.borrow_mut().define(param_name.clone(), arg_values[i].clone());
                        }

                        // 6. 执行函数体：保存当前解释器的环境，以便函数执行完后恢复
                        let previous_env = self.environment.clone();

                        // 切换解释器环境指向新的函数环境
                        self.environment = func_env;

                        // 执行函数体内的所有语句：execute_block_internal 负责遍历语句并执行，但不负责环境切换(我们已经切换了)
                        let result = self.execute_block_internal(&body);

                        // 7. 恢复环境：无论函数执行成功还是报错，都需要恢复环境，否则后续代码会在错误的上下文中运行
                        self.environment = previous_env;

                        // 8. 处理返回值 
                        match result {
                            // 情况 A: 函数自然执行结束（隐式返回最后一条语句的值）
                            Ok(val) => Ok(val), 

                            Err(e) => match e {
                                // 情况 B: 捕获到了显式的 return 语句，将特殊的 Return 错误"降级"为正常的返回值
                                RuntimeError::Return(return_value) => Ok(return_value),
        
                                // 情况 C: 真正的运行时错误，继续向上抛出
                                _ => Err(e),
                            },
                        }
                    }
                    // Todo: 支持 Native Function (如 print, clock)，在这里添加分支
                    // Value::NativeFunction { ... } => { ... }
                    _ => Err(RuntimeError::TypeError(format!("Can only call functions, got {}", callee))),
                }
            }
            Expr::List { elements } => {
                Ok(Value::List(self.evaluate_elements(elements)?))
            }
            Expr::Tuple { elements } => {
                Ok(Value::Tuple(self.evaluate_elements(elements)?))
            }
            Expr::Dict { elements } => {
                let mut dict = HashMap::new();
                for (k_expr, v_expr) in elements {
                    let key_val = self.evaluate(k_expr)?;

                    // Todo: 简化处理：将 Key 转为 String，允许特定类型作为 Key。
                    let key_str = key_val.to_string();

                    let val = self.evaluate(v_expr)?;

                    dict.insert(key_str, val);
                }
                Ok(Value::Dict(dict))
            }
            Expr::Identifier { name } => self
                .environment
                .borrow()
                .get(name)
                .ok_or_else(|| RuntimeError::UndefinedVariable(name.clone())),
            Expr::AssignOp { op, name, expr } => {
                // 1. 获取当前值
                let current_val = self
                    .environment
                    .borrow()
                    .get(name)
                    .ok_or_else(|| RuntimeError::UndefinedVariable(name.clone()))?;

                // 2. 计算右侧表达式
                let right_val = self.evaluate(expr)?;

                // 3. 根据 op 进行运算
                let new_val = match op {
                    Operator::Add => self.add_values(current_val, right_val)?,
                    Operator::Sub => {
                        self.check_number_operands(current_val, right_val, |a, b| {
                            Ok(Value::Number(a - b))
                        })?
                    }
                    Operator::Mul => {
                        self.check_number_operands(current_val, right_val, |a, b| {
                            Ok(Value::Number(a * b))
                        })?
                    }
                    Operator::Div => {
                        self.check_number_operands(current_val, right_val, |a, b| {
                            if b == 0.0 {
                                Err(RuntimeError::DivisionByZero)
                            } else {
                                Ok(Value::Number(a / b))
                            }
                        })?
                    }
                    _ => return Err(RuntimeError::Generic("Invalid assignment operator".into())),
                };

                // 4. 写回环境
                self.environment.borrow_mut().assign(name, new_val.clone());
                Ok(new_val)
            }
            Expr::Return { expr } => {
                let value = self.evaluate(expr)?;
                // 中断执行流
                Err(RuntimeError::Return(value))
            }
            Expr::VarDecl { name, initializer } => {
                let value = self.evaluate(initializer)?;
                // Shadowing (变量遮蔽) 在当前的作用域环境下插入，不改变父环境
                self.environment.borrow_mut().define(name.clone(), value);
                Ok(Value::Nil)
            },
				Expr::Print{expr} => {self.execute_print(expr)},
		  }
    }
	
    /// 执行代码块中的语句，创建新环境并处理作用域
    /// 
    /// # 参数
    /// * `statements` - 代码块中的语句列表
    /// * `new_env` - 新的环境，作为当前环境的子环境
    /// 
    /// # 返回值
    /// * `Ok(Value)` - 代码块中最后一条语句的执行结果
    fn execute_block(
        &mut self,
        statements: &[Expr],
        new_env: Environment,
    ) -> Result<Value, RuntimeError> {
        let previous = self.environment.clone();
        // 切换环境
        self.environment = Rc::new(RefCell::new(new_env));

        let result = self.execute_block_internal(statements);

        // 恢复环境 (很重要，否则作用域就乱了)
        self.environment = previous;
        result
    }

    /// 内部执行 Block 逻辑（不负责环境切换）
    /// 
    /// # 参数
    /// * `statements` - 代码块中的语句列表
    /// 
    /// # 返回值
    /// * `Ok(Value)` - 代码块中最后一条语句的执行结果
    fn execute_block_internal(&mut self, statements: &[Expr]) -> Result<Value, RuntimeError> {
        let mut result = Value::Nil;
        for stmt in statements {
            result = self.evaluate(stmt)?;
        }
        // 返回最后一条语句的值 (Implicit return)
        Ok(result)
    }

    /// 处理逻辑运算符短路求值
    /// 
    /// # 参数
    /// * `left` - 左侧表达式
    /// * `op` - 逻辑运算符 (AND 或 OR)
    /// * `right` - 右侧表达式
    fn evaluate_logical(
        &mut self,
        left: &Expr,
        op: &Operator,
        right: &Expr,
    ) -> Result<Value, RuntimeError> {
        let left_val = self.evaluate(left)?;

        if *op == Operator::LogicalOr {
            if left_val.is_truthy() {
                return Ok(left_val);
            }
        } else {
            // AND
            if !left_val.is_truthy() {
                return Ok(left_val);
            }
        } 

        self.evaluate(right)
    }

    /// 检查操作数是否为数字类型，并对其执行指定的操作
    /// 
    /// # 参数
    /// * `left` - 左操作数
    /// * `right` - 右操作数
    /// * `f` - 对两个数字执行的操作函数
    /// 
    /// # 返回值
    /// * `Ok(Value)` - 操作结果
    /// * `Err(RuntimeError)` - 类型错误
    fn check_number_operands<F>(
        &self,
        left: Value,
        right: Value,
        f: F,
    ) -> Result<Value, RuntimeError>
    where
        F: FnOnce(f64, f64) -> Result<Value, RuntimeError>, // Allow closure to return Result
    {
        match (left, right) {
            (Value::Number(n1), Value::Number(n2)) => f(n1, n2),
            _ => Err(RuntimeError::TypeError("Operands must be numbers.".into())),
        }
    }

    /// 处理两个值的加法运算，支持数字、字符串、列表、元组和字典的连接
    /// 
    /// # 参数
    /// * `left` - 左操作数
    /// * `right` - 右操作数
    /// 
    /// # 返回值
    /// * `Ok(Value)` - 连接或相加的结果
    fn add_values(&self, left: Value, right: Value) -> Result<Value, RuntimeError> {
            match (left, right) {
                (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),

                (Value::String(mut s1), Value::String(s2)) => {
                    // 左右值获取了所有权 复用内存
                    s1.push_str(&s2);
                    Ok(Value::String(s1))
                }

                //  列表、元组、字典
                (Value::List(mut list1), Value::List(list2)) => {
                    list1.extend(list2);
                    Ok(Value::List(list1))
                }
                (Value::Tuple(mut tuple1), Value::Tuple(tuple2)) => {
                    tuple1.extend(tuple2);
                    Ok(Value::Tuple(tuple1))
                }
				    (Value::Dict(mut dict1), Value::Dict(dict2)) => {
                    dict1.extend(dict2);
                    Ok(Value::Dict(dict1))
                }

                // edge case
                (l, r) => Err(RuntimeError::TypeError(format!(
                    "Binary operator '+' requires two numbers or two strings. Got {} and {}.",
                    l.type_name(),
                    r.type_name()
                ))),
            }
        }

    /// 对列表或元组中的表达式元素进行求值
    /// 
    /// 用于将表达式元素列表转换为值列表，支持列表和元组的元素初始化。
    /// 
    /// # 参数
    /// * `elements` - 表达式元素切片
    fn evaluate_elements(&mut self, elements: &[Expr]) -> Result<Vec<Value>, RuntimeError> {
        let mut values = Vec::new();
        for element in elements {
            values.push(self.evaluate(element)?);
        }
        Ok(values)
    }

	 /// 位运算辅助函数
    /// 
    /// 对两个表达式进行位运算操作，支持按位与、按位或操作
    /// 
    /// # 参数
    /// * `left_expr` - 左侧表达式
    /// * `right_expr` - 右侧表达式
    /// * `op` - 位运算操作函数
    /// 
    /// # 返回值
    /// * `Ok(Value)` - 位运算结果
    /// * `Err(RuntimeError)` - 类型错误（当操作数不是数字时）
    fn eval_bitwise<F>(&mut self, left_expr: &Expr, right_expr: &Expr, op: F) -> Result<Value, RuntimeError>
    where
        F: Fn(i64, i64) -> i64,
    {
        let l_val = self.evaluate(left_expr)?;
        let r_val = self.evaluate(right_expr)?;

        // 检查类型并转换
        match (l_val, r_val) {
            (Value::Number(n1), Value::Number(n2)) => {
                // f64 不支持位运算，必须转为 i64。
                // 这里会发生截断，例如 3.5 & 1 会变成 3 & 1。
                let i1 = n1 as i64;
                let i2 = n2 as i64;
                
                let result = op(i1, i2);
                
                // 转回 f64
                Ok(Value::Number(result as f64))
            }
            //  Rust：非数字无法进行位运算
            (l, r) => Err(RuntimeError::TypeError(format!(
                "Bitwise operands must be numbers. Got {} and {}.",
                l.type_name(),
                r.type_name()
            ))),
        }
    }
	 
	 fn execute_print(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
		let value = self.evaluate(expr)?;
		Ok(Value::Print(value.to_string()))
	 }
}



#[cfg(test)]
#[path = "tests/mod.rs"]
mod evaluate_tests;
