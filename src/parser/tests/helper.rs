use crate::ast::format::format_stmt;
use crate::parser::parse;
use crate::reader::Source;
use crate::tokenizer::tokenize;

/// 解析源码并返回格式化后的 AST 字符串列表
/// 只需比较字符串，不需手动构造复杂的 AST 结构体
pub fn parse_to_string(source: &str) -> Result<Vec<String>, String> {
    let source_obj = Source {
        contents: source.to_string(),
    };
    // 1. Tokenize
    let tokens = tokenize(source_obj).map_err(|e| format!("Scan Error: {}", e))?;

    // 2. Parse
    let ast = parse(tokens).map_err(|e| format!("Parse Error: {}", e))?;

    // 3. Format
    let output: Vec<String> = ast.body.iter().map(format_stmt).collect();

    Ok(output)
}

/// 断言解析语句成功（格式化结果匹配预期)
pub fn assert_parse(source: &str, expected: &str) {
    match parse_to_string(source) {
        Ok(stmts) => {
            // 将所有语句拼接对比 (忽略首尾可能的换行，但保留内部结构)
            let actual = stmts.join("\n");
            assert_eq!(
                actual, expected,
                "\nSource: {}\nExpected: {}\nActual:   {}",
                source, expected, actual
            );
        }
        Err(e) => panic!("Failed to parse: {}\nError: {}", source, e),
    }
}

/// 断言解析失败 (语法错误)
pub fn assert_error(source: &str, error_fragment: &str) {
    match parse_to_string(source) {
        Ok(_) => panic!(
            "Expected parse error for input: '{}', but succeeded.",
            source
        ),
        Err(e) => {
            if !e.contains(error_fragment) {
                panic!(
                    "Expected error containing '{}', but got: '{}'",
                    error_fragment, e
                );
            }
        }
    }
}
