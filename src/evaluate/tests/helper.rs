use crate::{
    evaluate::{Interpreter, Value},
    parser::parse,
    reader::Source,
    resolver::Resolver,
    tokenizer::tokenize,
};

// 由于修改 Interpreter 逻辑比较麻烦，我们采用 "全局变量探针" 的方式。
// 我们在测试脚本最后放一个全局变量读取，测试代码通过检查全局环境来验证结果。
// 但 Interpreter 的 environment 是私有的。
// 提供一个方法，使其能够返回最后一个表达式的值。

// 重新实现 eval helper，返回全局变量 "res" 的值
pub fn eval_res(source: &str) -> Result<Value, String> {
    let source_obj = Source {
        contents: source.to_string(),
    };
    let source = format!("var res = nil; {{ {} }}", source); // 包裹在块中，除了 var res
    // 哎呀，这样写稍微有点复杂，因为我们需要用户代码给 res 赋值。
    // 让我们约定：测试代码必须设置一个名为 'res' 的全局变量。

    let tokens = tokenize(source_obj).map_err(|e| format!("{:?}", e))?;
    let ast = parse(tokens).map_err(|e| format!("{:?}", e))?;

    let mut interpreter = Interpreter::new();
    let mut resolver = Resolver::new(&mut interpreter);
    resolver
        .resolve_stmts(&ast.body)
        .map_err(|e| format!("{:?}", e))?;

    interpreter.interpret(ast).map_err(|e| format!("{:?}", e))?;

    interpreter
        .get_global_value("res")
        .ok_or("Variable 'res' not found".to_string())
}

#[test]
fn test_arithmetic() {
    let code = "var res = 1 + 2 * 3;";
    assert_eq!(eval_res(code).unwrap(), Value::Number(7.0));
}

#[test]
fn test_closure() {
    let code = r#"
        fun makeCounter() {
            var i = 0;
            fun count() {
                i = i + 1;
                return i;
            }
            return count;
        }
        var counter = makeCounter();
        counter();
        var res = counter(); // res 应该是 2
    "#;
    assert_eq!(eval_res(code).unwrap(), Value::Number(2.0));
}

// Thinking

/// 编译并运行源代码，返回最后一个表达式语句的值，或者最后的状态
pub fn eval(source: &str) -> Result<Value, String> {
    let source_obj = Source {
        contents: source.to_string(),
    };
    // 1. Tokenize
    let tokens = tokenize(source_obj).map_err(|e| format!("Scan Error: {}", e))?;

    // 2. Parse
    let ast = parse(tokens).map_err(|e| format!("Parse Error: {}", e))?;

    // 3. Resolve
    let mut interpreter = Interpreter::new();
    let mut resolver = Resolver::new(&mut interpreter);

    // 注意：resolver 接收 &Vec<Stmt>
    resolver
        .resolve_stmts(&ast.body)
        .map_err(|e| format!("Resolution Error: {}", e))?;

    // 4. Interpret
    // 为了测试方便，我们需要 Interpreter 返回最后执行的那个值。
    // 但是现在的 interpret 返回的是 Value::Nil (因为是 Stmt 列表)。
    // 为了测试表达式求值，我们可以在测试代码中最后放一个变量读取或表达式。
    // 但由于 interpret 返回的是 Result<Value, ...> 且实现是 Ok(Value::Nil)，
    // 我们需要一种机制来捕获副作用，或者修改 interpret 逻辑。

    // 既然标准 Lox 语句不返回值，我们可以通过 "Side Effect" (副作用) 来测试。
    // 比如：定义一个变量，修改它，然后我们 inspect interpreter 的 environment。
    // 但为了黑盒测试，我们假设 interpret 能够返回最后一个表达式语句的值（如果你修改了 interpret），
    // 或者我们依赖 Print 的输出来验证（比较难自动化）。

    // **折衷方案**：我们修改一下 interpret 的测试逻辑，
    // 如果源代码只有一行且是 Expression Stmt，我们手动 evaluate 它。
    // 或者，更通用的：我们信任 interpret 的执行，
    // 对于测试用例，我们写成 `var res = ...;` 然后在 Rust 里无法直接读取 res。

    // **最佳方案**:
    // 我们的 evaluate 函数是私有的。为了测试，我们可以写一段特殊的胶水代码：
    // 在测试脚本最后写一个表达式，修改 Interpreter::interpret 让它返回最后一条 Stmt 的结果（如果是 ExprStmt）。

    // 这里假设你的 interpreter.interpret 依然返回 Ok(Value::Nil)。
    // 为了测试，建议在 Interpreter 增加一个 public 方法 `eval_expr_for_test`
    // 或者我们直接测试由脚本产生的 Side Effect (这比较难)。

    // **调整策略**：
    // 我们在测试中，通常希望测试 `1+1` 返回 `2`。
    // 让我们临时利用 `Interpreter` 的 `interpret` 返回值。
    // 如果你之前的 interpret 实现里，最后返回的是 Value::Nil，
    // 建议修改 interpret 逻辑：如果最后一条语句是 ExpressionStmt，返回它的值。

    interpreter
        .interpret(ast)
        .map_err(|e| format!("Runtime Error: {:?}", e))
}

// 注意：为了让下面的测试通过，你需要修改 src/interpreter.rs 中的 interpret 方法
// 让它记录最后一条语句的执行结果。
/*
    pub fn interpret(&mut self, ast: AST) -> Result<Value, RuntimeError> {
        let mut last_value = Value::Nil;
        for stmt in ast.body {
            match self.execute(&stmt) {
                // 如果 execute 能返回 Value 就好了，但现在返回 ()。
                // 这是一个架构权衡。为了测试，我们可以在 execute 的 Stmt::Expression 分支特殊处理。
            }
        }
        Ok(last_value)
    }
*/
