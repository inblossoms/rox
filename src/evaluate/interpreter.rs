use crate::ast::{AST, Expr, ExprId, Operator, Stmt};
use crate::evaluate::value::{RoxClass, RoxInstance};
use crate::evaluate::{environment::Environment, error::RuntimeError, value::Value};
use crate::tokenizer::Token;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/**
 * Thinking
 *
 * 双环境指针 (environment vs globals):
 *   解释器就像一个游标，environment 是游标当前的位置。
 *   globals 是游标的起点（根）。
 *   这解决了“如何在深层递归中快速访问全局变量”的问题，也确保了即使当前环境变了，全局状态依然存在且唯一。
 * Side Table 分离 (locals):
 *   我们没有把解析结果（距离）直接写在 AST 节点里（那样需要修改 AST 结构，且 AST 通常是不可变的）。
 *   我们也没有写在 Environment 里（Environment 是运行时数据）。
 *   我们将静态分析的结果存在 Interpreter 的这张 HashMap 中。这是一种经典的编译器设计模式，称为 Side Table。
 */

/// 解释器 (Interpreter)
///
/// 负责遍历 AST (抽象语法树)，执行语句 (Stmt) 并计算表达式 (Expr) 的值。
/// 运行时核心(v8)，维护程序运行时状态。
pub struct Interpreter {
    /// **当前**执行环境 (Current Environment)
    ///
    /// 指向当前代码正在执行的作用域。
    /// - 随着代码进入/退出块 (`{...}`) 或函数调用，这个字段会不断更新，
    ///   指向新的子环境或恢复到父环境。
    /// - 使用 `Rc<RefCell>` 是为了在函数闭包和解释器之间共享环境的所有权，
    ///   并支持修改（定义变量）。
    pub environment: Rc<RefCell<Environment>>,

    /// **全局**环境 (Global Environment)
    ///
    /// 永远指向最顶层的作用域。
    /// - 当解释器启动时，`environment` 和 `globals` 指向同一个环境。
    /// - 随着作用域加深，`environment` 会变化，但 `globals` 保持不变。
    /// - **用途**：
    ///   1. 用于定义和查找全局变量（无需递归回溯）。
    ///   2. 作为 `look_up_variable` 的兜底逻辑：如果 Resolver 没在 `locals`
    ///      中记录距离，则默认认为该变量是全局的。
    ///   3. 存放 Native Functions（如 `clock()`）。
    pub globals: Rc<RefCell<Environment>>,

    /// Side Table (Lookup Table)
    ///
    /// 存储由 `Resolver` (语义分析阶段) 计算出的静态作用域信息。
    /// - Resolver 会浏览 AST携带产生了一些额外信息。行为上不应塞进 AST，应当把这些信息存在独立的表里。AST 做为“主表”，locals Map 即 “侧表”。
    /// - **Key (`ExprId`)**: 源代码中某个具体位置的变量引用（AST 节点 ID）。
    /// - **Value (`usize`)**: 该变量定义在距离当前环境多少层之外 (Hops/Distance)。
    ///
    /// 解释器在执行 `Expr::Variable` 或 `Expr::Assign` 时，会先查这张表：
    /// - 如果查到了：使用 `environment.get_at(distance)` 精确获取变量（词法作用域）。
    /// - 如果没查到：假设是全局变量，去 `globals` 查找（动态作用域）。
    pub locals: HashMap<ExprId, usize>,
}

impl Interpreter {
    /// 创建一个新的解释器实例
    ///
    /// 初始化过程：
    /// 1. 创建一个全新的环境作为全局环境。
    /// 2. (可选) 在全局环境中注册原生函数 (Native Functions)。
    /// 3. 将当前环境 (`environment`) 和全局环境 (`globals`) 都指向这个新环境。
    pub fn new() -> Self {
        // 1. 创建根环境 (Global Scope)
        let globals = Rc::new(RefCell::new(Environment::new()));

        // TODO: 在这里注册原生函数，例如:
        // globals.borrow_mut().define("clock".to_string(), Value::NativeFn(...));

        Self {
            // 初始状态下，当前环境就是全局环境
            // clone() 增加引用计数，指向的依旧是同一块内存
            environment: globals.clone(),
            globals,
            locals: HashMap::new(),
        }
    }

    /// 入口函数：解释执行 AST
    pub fn interpret(&mut self, ast: AST) -> Result<Value, RuntimeError> {
        // ast.body 是 Vec<Stmt>
        for stmt in ast.body {
            match self.execute(&stmt) {
                Ok(_) => {} // 语句通常返回 Unit/Nil，继续执行下一条
                Err(e) => {
                    // 如果到了顶层还能捕获到 Break|Continue|Return，说明 Parser/Resolver 有 Bug
                    match e {
                        RuntimeError::Break => {
                            panic!("Critical Error: Parser allowed 'break' outside loop!")
                        }
                        RuntimeError::Continue => {
                            panic!("Critical Error: Parser allowed 'continue' outside loop!")
                        }
                        RuntimeError::Return(_) => {
                            panic!("Critical Error: Parser allowed 'return' outside function!")
                        }
                        _ => return Err(e),
                    }
                }
            }
        }
        Ok(Value::Nil) // 返回最后的状态，或者 Nil
    }

    // Statement Execution

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression { expr } => {
                self.evaluate(expr)?;
                Ok(())
            }
            Stmt::Print { expr } => {
                let value = self.evaluate(expr)?;
                println!("{}", value); // 副作用语句，将内容输出到 IO（控制台）
                Ok(()) // 表示语句执行完成，没有产生供后续计算的值
            }
            Stmt::VarDecl { name, initializer } => {
                let value = if let Some(expr) = initializer {
                    self.evaluate(expr)?
                } else {
                    Value::Nil
                };
                // 声明变量总是在当前环境
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), value);
                Ok(())
            }
            Stmt::Block { body } => {
                self.execute_block(body, Environment::with_enclosing(self.environment.clone()))?;
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.evaluate(condition)?.is_truthy() {
                    self.execute(then_branch)?;
                } else if let Some(else_b) = else_branch {
                    self.execute(else_b)?;
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                while self.evaluate(condition)?.is_truthy() {
                    match self.execute(body) {
                        Ok(_) => {}
                        Err(RuntimeError::Break) => break,
                        Err(RuntimeError::Continue) => continue,
                        Err(e) => return Err(e),
                    }
                }
                Ok(())
            }
            Stmt::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                let previous_env = self.environment.clone();
                // 创建新作用域 (init 变量)
                self.environment = Rc::new(RefCell::new(Environment::with_enclosing(
                    previous_env.clone(),
                )));

                if let Some(init) = initializer {
                    self.execute(init)?; // init 是 Stmt (VarDecl 或 ExprStmt)
                }

                // 循环：使用 try-finally 模式确保环境恢复
                let result = (|| -> Result<(), RuntimeError> {
                    loop {
                        // Check condition
                        if let Some(cond) = condition {
                            if !self.evaluate(cond)?.is_truthy() {
                                break;
                            }
                        }

                        // Run body
                        match self.execute(body) {
                            Ok(_) => {}
                            Err(RuntimeError::Break) => break,
                            Err(RuntimeError::Continue) => {
                                // Note：continue 也要执行 increment
                            }
                            Err(e) => return Err(e),
                        }

                        // Run increment
                        if let Some(incr) = increment {
                            self.evaluate(incr)?;
                        }
                    }
                    Ok(())
                })();

                self.environment = previous_env;
                result
            }
            Stmt::Function { name, params, body } => {
                let function = Value::Function {
                    name: name.lexeme.clone(),
                    // 适配 Value::Function 定义，可能需要转换，参数列表需要以 Vec<String> 存储
                    args: params.iter().map(|t| t.lexeme.clone()).collect(),
                    body: body.clone(), // body 是 Vec<Stmt>
                    closure: self.environment.clone(),
                };
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), function);
                Ok(())
            }
            Stmt::Class {
                name,
                superclass,
                methods,
            } => {
                // 处理父类
                let mut super_klass: Option<Rc<RefCell<RoxClass>>> = None;

                if let Some(expr) = superclass {
                    let val = self.evaluate(expr)?;
                    if let Value::Class(c) = val {
                        super_klass = Some(c);
                    } else {
                        return Err(RuntimeError::TypeError(
                            "Superclass must be a class.".into(),
                        ));
                    }
                }

                // Core：如果存在父类，我们需要创建一个环境闭包
                // Note：类定义时的环境就是它的闭包。
                //     如果我们用了 "super" 作用域，我们需要在 define class 之前处理环境。
                //     在这里的实现逻辑中，让 environment 指向一个新的环境，
                //     里面包含了 "super" -> superclass。
                if let Some(ref s) = super_klass {
                    self.environment = Rc::new(RefCell::new(Environment::with_enclosing(
                        self.environment.clone(),
                    )));
                    self.environment
                        .borrow_mut()
                        .define("super".to_string(), Value::Class(s.clone()));
                }

                // 将 AST 中的方法 (Stmt::Function) 转换为运行时 Value::Function
                let mut method_map = HashMap::new();
                for method in methods {
                    if let Stmt::Function {
                        name: m_name,
                        params,
                        body,
                    } = method
                    {
                        let function = Value::Function {
                            name: m_name.lexeme.clone(),
                            args: params.iter().map(|t| t.lexeme.clone()).collect(),
                            body: body.clone(),
                            closure: self.environment.clone(), // 闭包捕获当前环境
                        };
                        method_map.insert(m_name.lexeme.clone(), function);
                    }
                }

                // 创建 Class 对象
                let klass = RoxClass::new(name.lexeme.clone(), method_map, super_klass.clone());

                // 恢复环境 (弹出包含 super 的环境)
                if super_klass.is_some() {
                    // environment = environment.enclosing
                    let enclosing = self.environment.borrow().enclosing.clone();
                    self.environment = enclosing.unwrap();
                }

                // 定义到环境中
                self.environment.borrow_mut().define(
                    name.lexeme.clone(),
                    Value::Class(Rc::new(RefCell::new(klass))),
                );

                Ok(())
            }
            Stmt::Return { value, .. } => {
                let return_val = if let Some(expr) = value {
                    self.evaluate(expr)?
                } else {
                    Value::Nil
                };
                Err(RuntimeError::Return(return_val))
            }
            Stmt::Break => Err(RuntimeError::Break),
            Stmt::Continue => Err(RuntimeError::Continue),
            Stmt::Empty => Ok(()),
        }
    }

    // Expression Evaluation

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Number { value } => {
                let n = value
                    .parse::<f64>()
                    .map_err(|_| RuntimeError::Generic("Invalid number".into()))?;
                Ok(Value::Number(n))
            }
            Expr::String { value } => Ok(Value::String(value.clone())),
            Expr::Boolean { value } => Ok(Value::Boolean(*value)),
            Expr::Nil => Ok(Value::Nil),

            Expr::List { elements } => Ok(Value::List(self.evaluate_elements(elements)?)),
            Expr::Tuple { elements } => Ok(Value::Tuple(self.evaluate_elements(elements)?)),
            Expr::Dict { elements } => {
                let mut dict = HashMap::new();
                for (k, v) in elements {
                    let key = self.evaluate(k)?.to_string(); // Key 简化处理
                    let val = self.evaluate(v)?;
                    dict.insert(key, val);
                }
                Ok(Value::Dict(dict))
            }

            Expr::Variable { id, name } => self.look_up_variable(name, id),

            Expr::Assign { id, name, expr } => {
                let value = self.evaluate(expr)?;

                if let Some(&distance) = self.locals.get(id) {
                    // 本地赋值
                    self.environment
                        .borrow_mut()
                        .assign_at(distance, &name.lexeme, value.clone());
                } else {
                    // 全局赋值
                    let success = self
                        .globals
                        .borrow_mut()
                        .assign(&name.lexeme, value.clone());
                    if !success {
                        return Err(RuntimeError::UndefinedVariable(name.lexeme.clone()));
                    }
                }
                Ok(value)
            }

            Expr::AssignOp { id, name, op, expr } => {
                // 获取当前值 (Read)
                // 这里也应该走 look_up_variable，但 look_up 需要 ExprId
                // 如果你的 AST 中 AssignOp 有 ID，就这样写：
                let current_val = self.look_up_variable(name, id)?;

                // 计算 (Compute)
                let right_val = self.evaluate(expr)?;
                let new_val = match op {
                    Operator::Add => self.add_values(current_val, right_val)?,
                    // ... 复用之前的运算逻辑 ...
                    _ => return Err(RuntimeError::Generic("Invalid assign op".into())),
                };

                // 赋值回 (Write)
                if let Some(&distance) = self.locals.get(id) {
                    self.environment.borrow_mut().assign_at(
                        distance,
                        &name.lexeme,
                        new_val.clone(),
                    );
                } else {
                    self.globals
                        .borrow_mut()
                        .assign(&name.lexeme, new_val.clone());
                }
                Ok(new_val)
            }

            Expr::Logical { left, op, right } => {
                let left_val = self.evaluate(left)?;
                if *op == Operator::LogicalOr || *op == Operator::OrKeyword {
                    if left_val.is_truthy() {
                        return Ok(left_val);
                    }
                } else {
                    if !left_val.is_truthy() {
                        return Ok(left_val);
                    }
                }
                self.evaluate(right)
            }

            Expr::Binary { left, op, right } => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;

                match op {
                    Operator::Add => self.add_values(l, r),
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
                    Operator::Mod => self.check_number_operands(l, r, |a, b| {
                        if b == 0.0 {
                            Err(RuntimeError::DivisionByZero)
                        } else {
                            Ok(Value::Number(a.rem_euclid(b)))
                        }
                    }),

                    Operator::BitwiseAnd => self.eval_bitwise(left, right, |a, b| a & b),
                    Operator::BitwiseOr => self.eval_bitwise(left, right, |a, b| a | b),
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

            Expr::Call {
                id: _,
                callee,
                args,
            } => {
                // callee 可能是一个表达式：func()(1);
                // 如果 callee 是一个表达式: func()，则需要先求值
                // 检查 callee 的类型是否是 Expr::Variable，如果是 evaluate 内部会自动调用 look_up_variable
                let callee_value = self.evaluate(callee)?;
                //  let callee_value = self.look_up_variable(name_token, id)?;

                let mut arg_vals = Vec::new();
                for arg in args {
                    arg_vals.push(self.evaluate(arg)?);
                }

                match callee_value {
                    // 函数调用
                    Value::Function {
                        args: param_names,
                        body,
                        closure,
                        ..
                    } => {
                        if args.len() != param_names.len() {
                            return Err(RuntimeError::Generic("Arity mismatch".into()));
                        }

                        // 准备环境 (参数求值 + 绑定)
                        let mut arg_vals = Vec::new();
                        for arg in args {
                            arg_vals.push(self.evaluate(arg)?);
                        }

                        // 闭包环境
                        let func_env = Rc::new(RefCell::new(Environment::with_enclosing(closure)));
                        for (i, param_name) in param_names.iter().enumerate() {
                            func_env
                                .borrow_mut()
                                .define(param_name.clone(), arg_vals[i].clone());
                        }

                        // 执行 (委托给 helper)
                        let result = self.execute_block(&body, (*func_env).clone().into_inner());

                        match result {
                            Ok(_) => Ok(Value::Nil),
                            Err(RuntimeError::Return(v)) => Ok(v), // 把错误变回值
                            Err(e) => Err(e),                      // 其他错误继续抛出
                        }
                    }

                    // 类实例化
                    Value::Class(klass) => {
                        // 创建实例
                        let instance = Rc::new(RefCell::new(RoxInstance::new(klass.clone())));

                        // 是否有初始化器 (init)
                        let initializer = klass.borrow().find_method("init");

                        if let Some(init_method) = initializer {
                            // 有 init 方法：绑定并调用

                            // 绑定 'this' 到新创建的 instance
                            //     Note：init_method 是 Value::Function，bind 返回一个新的 Value::Function
                            let bound_init = init_method.bind(Value::Instance(instance.clone()));

                            // 解包绑定后的函数，准备执行
                            if let Value::Function {
                                args: param_names,
                                body,
                                closure,
                                ..
                            } = bound_init
                            {
                                // 检查参数数量
                                if arg_vals.len() != param_names.len() {
                                    return Err(RuntimeError::Generic(format!(
                                        "Expected {} arguments but got {}.",
                                        param_names.len(),
                                        arg_vals.len()
                                    )));
                                }

                                // 创建环境并绑定参数
                                let func_env =
                                    Rc::new(RefCell::new(Environment::with_enclosing(closure)));
                                for (i, param_name) in param_names.iter().enumerate() {
                                    func_env
                                        .borrow_mut()
                                        .define(param_name.clone(), arg_vals[i].clone());
                                }

                                // 执行 init 函数体
                                let result =
                                    self.execute_block(&body, (*func_env).clone().into_inner());

                                // 处理 init 的执行结果
                                match result {
                                    Ok(_) => {}                        // init 正常执行完毕
                                    Err(RuntimeError::Return(_)) => {} // 捕获 return; (Resolver 会确保 init 不能 return value)
                                    Err(e) => return Err(e),
                                }
                            }
                        } else {
                            // 没有 init 方法：参数必须为空
                            if !arg_vals.is_empty() {
                                return Err(RuntimeError::Generic(format!(
                                    "Expected 0 arguments but got {}.",
                                    arg_vals.len()
                                )));
                            }
                        }

                        // 无论是否有 init，实例化的结果永远是 instance 本身
                        Ok(Value::Instance(instance))
                    }

                    _ => Err(RuntimeError::TypeError("Can only call functions".into())),
                }
            }

            Expr::This { id, keyword } => self.look_up_variable(keyword, id),

            Expr::Get { object, name } => {
                let obj = self.evaluate(object)?;

                // 检查是否是实例
                if let Value::Instance(instance_rc) = obj {
                    let instance = instance_rc.borrow();

                    if let Some(value) = instance.fields.borrow().get(&name.lexeme) {
                        return Ok(value.clone());
                    }

                    if let Some(method) = instance.class.borrow().find_method(&name.lexeme) {
                        let bound_method = method.bind(Value::Instance(instance_rc.clone()));
                        return Ok(bound_method);
                    }

                    return Err(RuntimeError::Generic(format!(
                        "Undefined property '{}'.",
                        name.lexeme
                    )));
                }

                Err(RuntimeError::TypeError(
                    "Only instances have properties.".into(),
                ))
            }

            Expr::Set {
                object,
                name,
                value,
            } => {
                let obj = self.evaluate(object)?;

                // 检查是否是实例 只有实例才可以通过字段访问属性
                if let Value::Instance(instance_rc) = obj {
                    let val = self.evaluate(value)?;

                    // 写入字段
                    instance_rc
                        .borrow()
                        .fields
                        .borrow_mut()
                        .insert(name.lexeme.clone(), val.clone());

                    return Ok(val);
                }

                Err(RuntimeError::TypeError(
                    "Only instances have fields.".into(),
                ))
            }

            Expr::Grouping { expr } => self.evaluate(expr),
            Expr::Unary { op, expr } => {
                // 1. 先递归求右侧表达式的值
                let right = self.evaluate(expr)?;

                match op {
                    Operator::Not => Ok(Value::Boolean(!right.is_truthy())),

                    Operator::Sub => match right {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => Err(RuntimeError::TypeError("Operand must be a number.".into())),
                    },

                    // 逻辑：f64 -> i64 -> 按位取反 -> f64
                    Operator::BitwiseNot => match right {
                        Value::Number(n) => {
                            let int_val = n as i64;
                            Ok(Value::Number((!int_val) as f64))
                        }
                        _ => Err(RuntimeError::TypeError("Operand must be a number.".into())),
                    },

                    _ => Err(RuntimeError::Generic(format!(
                        "Invalid unary operator: {:?}",
                        op
                    ))),
                }
            }
            Expr::Super { id, method, .. } => {
                // 查找 "super" 获取父类对象
                // Resolver 保证了 "super" 在 distance 处
                let distance = *self.locals.get(id).unwrap();
                let superclass = self.environment.borrow().get_at(distance, "super").unwrap();

                // 查找 "this" 获取实例对象
                // Tip：Resolver 的环境链是 super -> this。
                //      所以 this 一定在 super 的下一层 (distance - 1)。
                let instance = self
                    .environment
                    .borrow()
                    .get_at(distance - 1, "this")
                    .unwrap();

                // 解包
                let super_klass = if let Value::Class(c) = superclass {
                    c
                } else {
                    panic!("Super not class")
                };

                // 查找并绑定方法
                if let Some(method_val) = super_klass.borrow().find_method(&method.lexeme) {
                    // 绑定到当前的 instance
                    return Ok(method_val.bind(instance)); // 绑定 this
                }

                Err(RuntimeError::UndefinedVariable(format!(
                    "Undefined property '{}'.",
                    method.lexeme
                )))
            }
        }
    }

    // Helper methods
    // Resolver 接口
    pub fn resolve(&mut self, expr_id: ExprId, depth: usize) {
        self.locals.insert(expr_id, depth);
    }

    ///  Core: 变量查找 (Variable Resolution)
    ///
    /// 连接 **Interpreter (运行时)** 和 **Resolver (静态分析)**。
    /// 该方法决定了变量是应该作为 **本地变量** (通过词法作用域链查找) 还是 **全局变量** (动态查找)。
    ///
    /// # 逻辑流程
    /// 1. **查表 (`locals`)**：使用 AST 节点的唯一 ID (`expr_id`) 在 `locals` 侧表中查找。
    ///    - 如果存在记录，说明 **Resolver** 在编译期已将其解析为本地变量，并计算出了它距离当前环境的深度 (`distance`)。
    ///    - 此时调用 `environment.get_at` 进行精确查找（跳过中间的父环境，直接去第 N 层取值）。
    /// 2. **查全局 (`globals`)**：如果侧表中没有记录，说明 Resolver 认为这是一个全局变量。
    ///    - 此时直接在 `globals` 环境中查找。
    ///
    /// # 参数
    /// * `name` - 变量名的 Token (用于报错时获取 lexeme 和行号)
    /// * `expr_id` - AST 节点的唯一 ID
    fn look_up_variable(&self, name: &Token, expr_id: &ExprId) -> Result<Value, RuntimeError> {
        if let Some(&distance) = self.locals.get(expr_id) {
            // 情况 A: 本地变量 (Lexical Scoping)
            self.environment
                .borrow()
                .get_at(distance, &name.lexeme)
                .ok_or_else(|| RuntimeError::UndefinedVariable(name.lexeme.clone()))
        } else {
            // 情况 B: 全局变量 (Dynamic Lookup)
            self.globals
                .borrow()
                .get(&name.lexeme)
                .ok_or_else(|| RuntimeError::UndefinedVariable(name.lexeme.clone()))
        }
    }

    /// 执行代码块并在指定环境中运行 (Block Execution)
    ///
    /// 负责管理作用域的 **进入** 和 **退出**。
    ///
    /// # 核心机制：Try-Finally 模拟
    /// 在解释器中，环境切换必须保证 **对称性**：进去了一个新环境，出来时必须恢复旧环境。
    /// 即使代码块内部发生了错误 (`Err`) 或控制流跳转 (`Return`/`Break`)，
    /// 环境恢复逻辑 `self.environment = previous` 也**必须**执行。
    ///
    /// 代码中使用立即执行闭包 `(|| { ... })()` 来模拟 `try-finally` 块，
    /// 确保环境恢复代码永远会被执行。
    ///
    /// # 参数
    /// * `statements` - 代码块内的语句列表
    /// * `new_env` - 要切换到的新环境 (通常父级指向当前的 `self.environment`)
    fn execute_block(
        &mut self,
        statements: &[Stmt],
        new_env: Environment,
    ) -> Result<(), RuntimeError> {
        let previous = self.environment.clone();

        self.environment = Rc::new(RefCell::new(new_env));

        // Execute
        // 使用闭包捕获执行结果，但不立即返回，以便恢复环境
        let result = (|| {
            for stmt in statements {
                self.execute(stmt)?;
            }
            Ok(())
        })();

        // 无论 result 是 Ok 还是 Err，这一步都会执行
        self.environment = previous;

        result
    }

    /// 批量求值表达式列表 (Batch Evaluation)
    ///
    /// 通用辅助函数，用于将 `Vec<Expr>` 转换为 `Vec<Value>`。
    /// 保证了表达式是从左到右依次求值的。
    ///
    /// # 适用场景
    /// * 列表/数组字面量: `[1, 2, a]`
    /// * 元组字面量: `(1, 2)`
    /// * 函数调用参数: `func(a, b, c)`
    ///
    /// # 参数
    /// * `elements` - 表达式切片
    fn evaluate_elements(&mut self, elements: &[Expr]) -> Result<Vec<Value>, RuntimeError> {
        let mut res = Vec::new();
        for e in elements {
            res.push(self.evaluate(e)?);
        }
        Ok(res)
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
    fn eval_bitwise<F>(
        &mut self,
        left_expr: &Expr,
        right_expr: &Expr,
        op: F,
    ) -> Result<Value, RuntimeError>
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

    /// 获取全局变量的值（仅在测试时可用）
    #[cfg(test)]
    pub fn get_global_value(&self, name: &str) -> Option<Value> {
        self.globals.borrow().get(name)
    }
}
