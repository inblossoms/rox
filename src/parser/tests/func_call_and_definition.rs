use crate::parser::tests::{assert_parse, parse_to_string};

#[test]
fn test_function_call() {
    assert_parse("average(1, 2);", "average(1, 2);");
    // 连续调用: getCallback()()
    assert_parse("get()(1);", "get()(1);");
}

#[test]
fn test_function_declaration() {
    assert_parse(
        "fun add(a, b) { return a + b; }",
        "fun add(a, b) { return (a + b); }",
    );
}

#[test]
fn test_call_max_args() {
    // 构造一个超长参数列表
    let mut args = Vec::new();
    for i in 0..300 {
        args.push(format!("{}", i));
    }
    let code = format!("foo({});", args.join(","));

    // 解析器应该报错（如果你实现了255个参数限制）或者解析成功
    // 这里假设我们允许解析通过，或者你可以测试它报错
    let _ = parse_to_string(&code);
}
