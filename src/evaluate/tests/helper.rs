use crate::{
    evaluate::{Interpreter, Value},
    parser::parse,
    reader::Source,
    resolver::Resolver,
    tokenizer::tokenize,
};

/// 编译并运行源代码，返回最后一个表达式语句的值，或者最后的状态
pub fn eval_res(source: &str) -> Result<Value, String> {
    let source_obj = Source {
        contents: source.to_string(),
    };

    let tokens = tokenize(source_obj).map_err(|e| format!("{:?}", e))?;
    let ast = parse(tokens).map_err(|e| format!("{:?}", e))?;

    let mut interpreter = Interpreter::new();
    let mut resolver = Resolver::new(&mut interpreter);
    resolver
        .resolve_stmts(&ast.body)
        .map_err(|e| format!("{:?}", e))?;

    interpreter.interpret(ast).map_err(|e| format!("{:?}", e))?;

    // interpreter.interpret 最后会返回 Ok(Value::Nil)，
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

// 在实现辅助函数之前，首先要明白一个点是测试的目的是什么？
// 为了测试表达式的求值结果值是否正确。

// 为了方便测试，需要 Interpreter 返回最后执行的那个值。
// 现在的 interpret 返回的是 Value::Nil (因为是 Stmt 列表)。
// 为了测试表达式求值，可以试着在测试代码中最后放一个变量读取或表达式。
// 但由于 interpret 返回的是 Result<Value, ...> 且实现是 Ok(Value::Nil)，
// 需要一种机制来捕获副作用<1>，或者修改 interpret 逻辑<2>。
//
// <2>
// 通过 "Side Effect" (副作用) 来测试。
// 比如：定义一个变量，修改它，然后我们 inspect interpreter 的 environment。
// 但为了黑盒测试，我们假设 interpret 能够返回最后一个表达式语句的值，
// 或者我们依赖 Print 的输出来验证（比较难自动化）。
//
// 修改 interpret 的测试逻辑，
// 如果源代码只有一行且是 Expression Stmt，我们手动 evaluate 它。
//    或者，更通用的：信任 interpret 的执行，
//    对于测试用例，写成 `var res = ...;` 但是在 Rust 里无法直接读取 res。
//
// <1>
// 我们的 evaluate 函数是私有的。为了测试，我们可以写一段特殊的胶水代码：
// 在测试脚本最后写一个表达式，采用 "全局变量探针" 的方式。
// 在测试脚本最后放一个全局变量读取，测试代码通过检查全局环境来验证结果。
// 但 Interpreter 的 environment 是私有的。
// 所以提供一个方法: get_global_value，使其能够返回最后一个表达式的值。
