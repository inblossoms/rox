use crate::ast::{AST, Expr, Operator};
use crate::evaluate::environment::Environment;
use crate::evaluate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;


#[derive(Debug)]
pub struct Error {}

// 运行时错误
#[derive(Debug)]
#[allow(dead_code)]
pub enum RuntimeError {

    Generic(String),
    UndefinedVariable(String),
    TypeError(String),
    DivisionByZero,
    Return(Value), // 携带返回值
    Break,
    Continue,
}

// 解释器结构体
pub struct Interpreter {
    // 当前环境指针
    pub environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    // 入口函数
    pub fn interpret(&mut self, ast: AST) -> Result<Value, RuntimeError> {
        if let Some(expr) = ast.top {
            // 如果顶层节点是 Block（由 parse_program 生成），
            // 需要“解包”这个 Block，直接在当前环境执行其中的语句，
            // 而不是调用 self.evaluate(&expr) —— 因为后者会创建一个临时的子作用域。
            if let Expr::Block { body } = expr {
                let mut result = Value::Nil;
                for stmt in body {
                    result = self.evaluate(&stmt)?;
                }
                return Ok(result);
            }

            // 如果不是 Block，则正常求值
            return self.evaluate(&expr);
        }
        Ok(Value::Nil)
    }

    // 递归求值核心函数
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
                // 特殊处理逻辑运算符 (短路求值)
                if *op == Operator::And || *op == Operator::Or {
                    return self.evaluate_logical(left, op, right);
                }

                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;

                match op {
                    // 数学运算
                    Operator::Add => match (l, r) {
                        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
                        (Value::String(s1), Value::String(s2)) => {
                            Ok(Value::String(format!("{}{}", s1, s2)))
                        }
                        // 甚至可以支持 字符串 + 数字
                        (Value::String(s1), r_val) => Ok(Value::String(format!("{}{}", s1, r_val))),
                        _ => Err(RuntimeError::TypeError(
                            "Operands must be two numbers or two strings".into(),
                        )),
                    },
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

                    // 相等运算 (支持所有类型)
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
                    // 【严谨】使用 assign，递归查找。
                    let success = self.environment.borrow_mut().assign(name, value.clone());
                    
                    if success {
                        Ok(value)
                    } else {
                        // 【严谨】如果找不到变量，报错！而不是自动创建。
                        // 这样避免了隐式全局变量，强制用户先 var 声明。
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
                // 循环条件求值
                while self.evaluate(condition)?.is_truthy() {
                    // 执行循环体
                    match self.evaluate(body) {
                        Ok(val) => result = val,
                        Err(e) => match e {
                            // 捕获 Break: 退出循环，返回 Ok
                            RuntimeError::Break => break,
                            // 捕获 Continue: 忽略剩余部分，进行下一次循环
                            RuntimeError::Continue => continue,
                            // 捕获 Return: 继续向上抛出，直到遇到函数边界
                            RuntimeError::Return(v) => return Err(RuntimeError::Return(v)),
                            // 其他错误: 也是直接抛出
                            _ => return Err(e),
                        },
                    }
                }
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
                // 将函数定义在当前环境中
                self.environment.borrow_mut().define(name.clone(), function);
                Ok(Value::Nil)
            }
            Expr::Call { name, args } => {
                // 1. 获取被调用对象 (Callee)
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
                        // 2. 检查参数数量 (Arity Check)
                        if args.len() != param_names.len() {
                            return Err(RuntimeError::Generic(format!(
                                "Expected {} arguments but got {}.",
                                param_names.len(),
                                args.len()
                            )));
                        }

                        // 3. 求值实参 (Evaluate Arguments)
                        // 注意：参数必须在 *当前环境* (调用者的环境) 下求值
                        let mut arg_values = Vec::new();
                        for arg in args {
                            arg_values.push(self.evaluate(arg)?);
                        }

                        // 4. 创建函数作用域 (Scope Creation)
                        // 关键点：新环境的父环境必须是 **closure** (定义时的环境)，
                        // 而不是 self.environment (调用时的环境)。
                        // 只有这样才能实现词法作用域 (Lexical Scoping)。
                        let func_env = Rc::new(RefCell::new(Environment::with_enclosing(closure)));

                        // 5. 绑定参数 (Parameter Binding)
                        for (i, param_name) in param_names.iter().enumerate() {
                            // 将计算好的实参值，绑定到新环境中的形参名上
                            func_env.borrow_mut().define(param_name.clone(), arg_values[i].clone());
                        }

                        // 6. 执行函数体 (Execution)
                        // 保存当前解释器的环境，以便函数执行完后恢复
                        let previous_env = self.environment.clone();

                        // 切换解释器环境指向新的函数环境
                        self.environment = func_env;

                        // 执行函数体内的所有语句
                        // execute_block_internal 负责遍历语句并执行，但不负责环境切换(我们已经切换了)
                        let result = self.execute_block_internal(&body);

                        // 7. 恢复环境 (Restore Environment)
                        // 无论函数执行成功还是报错，都必须恢复环境，否则后续代码会在错误的上下文中运行
                        self.environment = previous_env;

                        // 8. 处理返回值 (Handle Return Signal)
                        match result {
                            // 情况 A: 函数自然执行结束（隐式返回最后一条语句的值，类似 Rust/Ruby）
                            Ok(val) => Ok(val), 

                            Err(e) => match e {
                                // 情况 B: 捕获到了显式的 return 语句
                                // 将特殊的 Return 错误"降级"为正常的返回值
                                RuntimeError::Return(return_value) => Ok(return_value),
        
                                // 情况 C: 真正的运行时错误（如类型错误、除以零），继续向上抛出
                                _ => Err(e),
                            },
                        }
                    }
                    // 如果未来支持 Native Function (如 print, clock)，可以在这里加分支
                    // Value::NativeFunction { ... } => { ... }
                    _ => Err(RuntimeError::TypeError(format!("Can only call functions, got {}", callee))),
                }
            }
            Expr::List { elements } => {
                let mut values = Vec::new();
                for element in elements {
                    values.push(self.evaluate(element)?);
                }
                Ok(Value::List(values))
            }
            Expr::Tuple { elements } => {
                let mut values = Vec::new();
                for element in elements {
                    values.push(self.evaluate(element)?);
                }
                Ok(Value::Tuple(values))
            }
            Expr::Dict { elements } => {
                let mut dict = HashMap::new();
                for (k_expr, v_expr) in elements {
                    // 计算 key
                    let key_val = self.evaluate(k_expr)?;
                    // 这里简化处理：将所有 Key 转为 String。
                    // 实际语言中可能只允许特定类型作为 Key。
                    let key_str = key_val.to_string();

                    // 计算 value
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
                // 第一步：获取当前值
                let current_val = self
                    .environment
                    .borrow()
                    .get(name)
                    .ok_or_else(|| RuntimeError::UndefinedVariable(name.clone()))?;

                // 第二步：计算右侧表达式
                let right_val = self.evaluate(expr)?;

                // 第三步：根据 op 进行运算 (重用 Binary 的逻辑或单独实现)
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

                // 第四步：写回环境
                self.environment.borrow_mut().assign(name, new_val.clone());
                Ok(new_val)
            }
            Expr::Return { expr } => {
                let value = self.evaluate(expr)?;
                // 抛出 Return 信号，中断执行流
                Err(RuntimeError::Return(value))
            }
            Expr::Var { name, initializer } => {
                let value = self.evaluate(initializer)?;
                // 【严谨】使用 define，强制在【当前】环境插入，绝不触碰父环境
                // 这就实现了 Shadowing (遮蔽)
                self.environment.borrow_mut().define(name.clone(), value);
                Ok(Value::Nil)
            },
            Expr::Break => Err(RuntimeError::Break),
            Expr::Continue => Err(RuntimeError::Continue),
		  }
    }
	
    // 辅助：执行代码块
    fn execute_block(
        &mut self,
        statements: &[Expr],
        new_env: Environment,
    ) -> Result<Value, RuntimeError> {
        let previous = self.environment.clone();
        // 切换环境
        self.environment = Rc::new(RefCell::new(new_env));

        let result = self.execute_block_internal(statements);

        // 恢复环境 (这一步非常重要，否则作用域就乱了)
        self.environment = previous;
        result
    }

    // 内部执行 Block 逻辑（不负责环境切换）
    fn execute_block_internal(&mut self, statements: &[Expr]) -> Result<Value, RuntimeError> {
        let mut result = Value::Nil;
        for stmt in statements {
            result = self.evaluate(stmt)?;
        }
        // 返回最后一条语句的值 (Implicit return)
        Ok(result)
    }

    // 辅助：逻辑运算短路
    fn evaluate_logical(
        &mut self,
        left: &Expr,
        op: &Operator,
        right: &Expr,
    ) -> Result<Value, RuntimeError> {
        let left_val = self.evaluate(left)?;

        if *op == Operator::Or {
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

    // 辅助：检查数字操作数
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

fn add_values(&self, left: Value, right: Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),

            (Value::String(mut s1), Value::String(s2)) => {
                // 左右值已经获取了所有权 可以直接复用内存
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

            // 类型无法匹配
            (l, r) => Err(RuntimeError::TypeError(format!(
                "Binary operator '+' requires two numbers or two strings. Got {} and {}.",
                l.type_name(),
                r.type_name()
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Expr, Operator};

    fn create_test_interpreter() -> Interpreter {
        Interpreter::new()
    }


    #[test]
    fn test_number_evaluation() {
        let mut interpreter = create_test_interpreter();
        let expr = Expr::Number { value: "42".to_string() };
        let result = interpreter.evaluate(&expr).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_string_evaluation() {
        let mut interpreter = create_test_interpreter();
        let expr = Expr::String { value: "hello".to_string() };
        let result = interpreter.evaluate(&expr).unwrap();
        assert_eq!(result, Value::String("hello".to_string()));
    }

    #[test]
    fn test_boolean_evaluation() {
        let mut interpreter = create_test_interpreter();
        let expr = Expr::Boolean { value: true };
        let result = interpreter.evaluate(&expr).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_nil_evaluation() {
        let mut interpreter = create_test_interpreter();
        let expr = Expr::Nil;
        let result = interpreter.evaluate(&expr).unwrap();
        assert_eq!(result, Value::Nil);
    }

    #[test]
    fn test_unary_operations() {
        let mut interpreter = create_test_interpreter();
        
        // Test unary minus
        let expr = Expr::Unary {
            op: Operator::Sub,
            expr: Box::new(Expr::Number { value: "42".to_string() }),
        };
        let result = interpreter.evaluate(&expr).unwrap();
        assert_eq!(result, Value::Number(-42.0));

        // Test logical NOT
        let expr = Expr::Unary {
            op: Operator::Not,
            expr: Box::new(Expr::Boolean { value: true }),
        };
        let result = interpreter.evaluate(&expr).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_binary_arithmetic_operations() {
        let mut interpreter = create_test_interpreter();
        
        // Test addition
        let expr = Expr::Binary {
            left: Box::new(Expr::Number { value: "10".to_string() }),
            op: Operator::Add,
            right: Box::new(Expr::Number { value: "5".to_string() }),
        };
        let result = interpreter.evaluate(&expr).unwrap();
        assert_eq!(result, Value::Number(15.0));

        // Test subtraction
        let expr = Expr::Binary {
            left: Box::new(Expr::Number { value: "10".to_string() }),
            op: Operator::Sub,
            right: Box::new(Expr::Number { value: "5".to_string() }),
        };
        let result = interpreter.evaluate(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));

        // Test multiplication
        let expr = Expr::Binary {
            left: Box::new(Expr::Number { value: "10".to_string() }),
            op: Operator::Mul,
            right: Box::new(Expr::Number { value: "5".to_string() }),
        };
        let result = interpreter.evaluate(&expr).unwrap();
        assert_eq!(result, Value::Number(50.0));

        // Test division
        let expr = Expr::Binary {
            left: Box::new(Expr::Number { value: "10".to_string() }),
            op: Operator::Div,
            right: Box::new(Expr::Number { value: "2".to_string() }),
        };
        let result = interpreter.evaluate(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));

        // Test division by zero
        let expr = Expr::Binary {
            left: Box::new(Expr::Number { value: "10".to_string() }),
            op: Operator::Div,
            right: Box::new(Expr::Number { value: "0".to_string() }),
        };
        let result = interpreter.evaluate(&expr);
        assert!(matches!(result, Err(RuntimeError::DivisionByZero)));
    }

    #[test]
    fn test_binary_comparison_operations() {
        let mut interpreter = create_test_interpreter();
        
        // Test greater than
        let expr = Expr::Binary {
            left: Box::new(Expr::Number { value: "10".to_string() }),
            op: Operator::Greater,
            right: Box::new(Expr::Number { value: "5".to_string() }),
        };
        let result = interpreter.evaluate(&expr).unwrap();
        assert_eq!(result, Value::Boolean(true));

        // Test less than or equal
        let expr = Expr::Binary {
            left: Box::new(Expr::Number { value: "5".to_string() }),
            op: Operator::LessEqual,
            right: Box::new(Expr::Number { value: "5".to_string() }),
        };
        let result = interpreter.evaluate(&expr).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_binary_equality_operations() {
        let mut interpreter = create_test_interpreter();
        
        // Test equality
        let expr = Expr::Binary {
            left: Box::new(Expr::Number { value: "5".to_string() }),
            op: Operator::Equal,
            right: Box::new(Expr::Number { value: "5".to_string() }),
        };
        let result = interpreter.evaluate(&expr).unwrap();
        assert_eq!(result, Value::Boolean(true));

        // Test inequality
        let expr = Expr::Binary {
            left: Box::new(Expr::Number { value: "5".to_string() }),
            op: Operator::NotEqual,
            right: Box::new(Expr::Number { value: "10".to_string() }),
        };
        let result = interpreter.evaluate(&expr).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_logical_operations() {
        let mut interpreter = create_test_interpreter();
        
        // Test AND short-circuit
        let expr = Expr::Binary {
            left: Box::new(Expr::Boolean { value: false }),
            op: Operator::And,
            right: Box::new(Expr::Number { value: "100".to_string() }), // This shouldn't be evaluated
        };
        let result = interpreter.evaluate(&expr).unwrap();
        assert_eq!(result, Value::Boolean(false));

        // Test OR short-circuit
        let expr = Expr::Binary {
            left: Box::new(Expr::Boolean { value: true }),
            op: Operator::Or,
            right: Box::new(Expr::Number { value: "100".to_string() }), // This shouldn't be evaluated
        };
        let result = interpreter.evaluate(&expr).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_variable_definition_and_access() {
        let mut interpreter = create_test_interpreter();
        
        // Define a variable
        let assign_expr = Expr::Var {
            name: "x".to_string(),
            initializer: Box::new(Expr::Number { value: "42".to_string() }),
        };
        interpreter.evaluate(&assign_expr).unwrap();
        
        // Access the variable
        let var_expr = Expr::Variable { name: "x".to_string() };
        let result = interpreter.evaluate(&var_expr).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_variable_assignment() {
        let mut interpreter = create_test_interpreter();
        
        // Define and assign
        let assign_expr = Expr::Var {
            name: "x".to_string(),
            initializer: Box::new(Expr::Number { value: "42".to_string() }),
        };
        interpreter.evaluate(&assign_expr).unwrap();
        
        // Reassign
        let reassign_expr = Expr::Var {
            name: "x".to_string(),
            initializer: Box::new(Expr::Number { value: "100".to_string() }),
        };
        interpreter.evaluate(&reassign_expr).unwrap();
        
        // Check new value
        let var_expr = Expr::Variable { name: "x".to_string() };
        let result = interpreter.evaluate(&var_expr).unwrap();
        assert_eq!(result, Value::Number(100.0));
    }

    #[test]
    fn test_undefined_variable() {
        let mut interpreter = create_test_interpreter();
        
        let var_expr = Expr::Variable { name: "undefined_var".to_string() };
        let result = interpreter.evaluate(&var_expr);
        assert!(matches!(result, Err(RuntimeError::UndefinedVariable(_))));
    }

    #[test]
    fn test_block_scoping() {
        let mut interpreter = create_test_interpreter();
        
        // Define variable in outer scope
        let outer_assign = Expr::Var {
            name: "x".to_string(),
            initializer: Box::new(Expr::Number { value: "10".to_string() }),
        };
        interpreter.evaluate(&outer_assign).unwrap();
        
        // Create a block with inner scope
        let inner_assign = Expr::Var {
            name: "x".to_string(), // Same name as outer scope
            initializer: Box::new(Expr::Number { value: "20".to_string() }),
        };
        let block = Expr::Block {
            body: vec![inner_assign],
        };
        
        interpreter.evaluate(&block).unwrap();
        
        // Check that outer scope variable is unchanged
        let var_expr = Expr::Variable { name: "x".to_string() };
        let result = interpreter.evaluate(&var_expr).unwrap();
        assert_eq!(result, Value::Number(10.0)); // Should still be 10, not 20
    }

    #[test]
    fn test_if_statement() {
        let mut interpreter = create_test_interpreter();
        
        // Test true branch
        let if_expr = Expr::If {
            condition: Box::new(Expr::Boolean { value: true }),
            then_branch: Box::new(Expr::Number { value: "42".to_string() }),
            else_branch: Some(Box::new(Expr::Number { value: "0".to_string() })),
        };
        let result = interpreter.evaluate(&if_expr).unwrap();
        assert_eq!(result, Value::Number(42.0));

        // Test false branch
        let if_expr = Expr::If {
            condition: Box::new(Expr::Boolean { value: false }),
            then_branch: Box::new(Expr::Number { value: "42".to_string() }),
            else_branch: Some(Box::new(Expr::Number { value: "0".to_string() })),
        };
        let result = interpreter.evaluate(&if_expr).unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_while_loop() {
        let mut interpreter = create_test_interpreter();
        
        // Simple counter test: while x < 5, x = x + 1
        // First set x = 0
        let init = Expr::Var {
            name: "x".to_string(),
            initializer: Box::new(Expr::Number { value: "0".to_string() }),
        };
        interpreter.evaluate(&init).unwrap();
        
        // Create the loop: while x < 5 { x = x + 1 }
        let loop_expr = Expr::While {
            condition: Box::new(Expr::Binary {
                left: Box::new(Expr::Variable { name: "x".to_string() }),
                op: Operator::Less,
                right: Box::new(Expr::Number { value: "5".to_string() }),
            }),
            body: Box::new(Expr::Assign {
                name: "x".to_string(),
                expr: Box::new(Expr::Binary {
                    left: Box::new(Expr::Variable { name: "x".to_string() }),
                    op: Operator::Add,
                    right: Box::new(Expr::Number { value: "1".to_string() }),
                }),
            }),
        };
        
        interpreter.evaluate(&loop_expr).unwrap();
        
        // Check that x is now 5
        let var_expr = Expr::Variable { name: "x".to_string() };
        let result = interpreter.evaluate(&var_expr).unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    #[test]
    fn test_function_definition_and_call() {
        let mut interpreter = create_test_interpreter();
        
        // Define a function: fun add(x, y) { x + y }
        let func_def = Expr::Function {
            name: "add".to_string(),
            args: vec!["x".to_string(), "y".to_string()],
            body: vec![Expr::Binary {
                left: Box::new(Expr::Variable { name: "x".to_string() }),
                op: Operator::Add,
                right: Box::new(Expr::Variable { name: "y".to_string() }),
            }],
        };
        interpreter.evaluate(&func_def).unwrap();
        
        // Call the function: add(3, 5)
        let call = Expr::Call {
            name: "add".to_string(),
            args: vec![
                Expr::Number { value: "3".to_string() },
                Expr::Number { value: "5".to_string() },
            ],
        };
        let result = interpreter.evaluate(&call).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    #[test]
    fn test_function_with_closure() {
        let mut interpreter = create_test_interpreter();
        
        // Define outer variable
        let outer_def = Expr::Var {
            name: "x".to_string(),
            initializer: Box::new(Expr::Number { value: "10".to_string() }),
        };
        interpreter.evaluate(&outer_def).unwrap();
        
        // Define function that captures x
        let func_def = Expr::Function {
            name: "get_x".to_string(),
            args: vec![],
            body: vec![Expr::Variable { name: "x".to_string() }],
        };
        interpreter.evaluate(&func_def).unwrap();
        
        // Call the function
        let call = Expr::Call {
            name: "get_x".to_string(),
            args: vec![],
        };
        let result = interpreter.evaluate(&call).unwrap();
        assert_eq!(result, Value::Number(10.0));
    }

    #[test]
    fn test_list_operations() {
        let mut interpreter = create_test_interpreter();
        
        // Create a list: [1, 2, 3]
        let list_expr = Expr::List {
            elements: vec![
                Expr::Number { value: "1".to_string() },
                Expr::Number { value: "2".to_string() },
                Expr::Number { value: "3".to_string() },
            ],
        };
        let result = interpreter.evaluate(&list_expr).unwrap();
        assert_eq!(result, Value::List(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]));
    }

    #[test]
    fn test_tuple_operations() {
        let mut interpreter = create_test_interpreter();
        
        // Create a tuple: (1, "hello")
        let tuple_expr = Expr::Tuple {
            elements: vec![
                Expr::Number { value: "1".to_string() },
                Expr::String { value: "hello".to_string() },
            ],
        };
        let result = interpreter.evaluate(&tuple_expr).unwrap();
        assert_eq!(result, Value::Tuple(vec![
            Value::Number(1.0),
            Value::String("hello".to_string()),
        ]));
    }

    #[test]
    fn test_dict_operations() {
        let mut interpreter = create_test_interpreter();
        
        // Create a dict: {"key1": 1, "key2": "value"}
        // Note: This requires more complex AST setup for key-value pairs
        // For now, testing a simple dict creation
        let dict_expr = Expr::Dict {
            elements: vec![
                (
                    Expr::String { value: "key1".to_string() },
                    Expr::Number { value: "1".to_string() },
                ),
                (
                    Expr::String { value: "key2".to_string() },
                    Expr::String { value: "value".to_string() },
                ),
            ],
        };
        let result = interpreter.evaluate(&dict_expr).unwrap();
        let mut expected_dict = HashMap::new();
        expected_dict.insert("key1".to_string(), Value::Number(1.0));
        expected_dict.insert("key2".to_string(), Value::String("value".to_string()));
        assert_eq!(result, Value::Dict(expected_dict));
    }

    #[test]
    fn test_return_statement() {
        let mut interpreter = create_test_interpreter();
        
        // Function with return: fn test() { return 42; 100 }
        let func_def = Expr::Function {
            name: "test".to_string(),
            args: vec![],
            body: vec![
                Expr::Return { expr: Box::new(Expr::Number { value: "42".to_string() }) },
                Expr::Number { value: "100".to_string() }, // This should not be executed
            ],
        };
        interpreter.evaluate(&func_def).unwrap();
        
        // Call the function
        let call = Expr::Call {
            name: "test".to_string(),
            args: vec![],
        };
        let result = interpreter.evaluate(&call).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_compound_assignment() {
        let mut interpreter = create_test_interpreter();
        
        // Set x = 10
        let init = Expr::Var {
            name: "x".to_string(),
            initializer: Box::new(Expr::Number { value: "10".to_string() }),
        };
        interpreter.evaluate(&init).unwrap();
        
        // x += 5 (should result in 15)
        let compound_assign = Expr::AssignOp {
            op: Operator::Add,
            name: "x".to_string(),
            expr: Box::new(Expr::Number { value: "5".to_string() }),
        };
        let result = interpreter.evaluate(&compound_assign).unwrap();
        assert_eq!(result, Value::Number(15.0));
        
        // Verify the variable was updated
        let var_check = Expr::Variable { name: "x".to_string() };
        let final_value = interpreter.evaluate(&var_check).unwrap();
        assert_eq!(final_value, Value::Number(15.0));
    }
}
