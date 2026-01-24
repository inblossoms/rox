use crate::{
    error::RoxError,
    evaluate::error::RuntimeError, // tokenizer 具体错误信息
    parser::error::Error as ParseError,
    tokenizer::{Error as ScanError, ScanError as SingleScanError},
};
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
    term::termcolor::{ColorChoice, StandardStream},
    term::{self, Config},
};
use std::ops::Range;

/// 入口：打印修饰后的错误信息
///
/// # 参数
/// * `file_name`: 文件名 (如 "script.rox" 或 "<stdin>")
/// * `source_code`: 完整的源代码字符串
/// * `error`: 捕获到的 RoxError
pub fn print_diagnostic(file_name: &str, source_code: &str, error: &RoxError) {
    // 1. 创建文件数据库
    let mut files = SimpleFiles::new();
    let file_id = files.add(file_name, source_code);

    // 2. 将 RoxError 转换为 Codespan 的 Diagnostic
    let diagnostic = match error {
        RoxError::Tokenize(e) => map_scan_error(file_id, source_code, e),
        RoxError::Parse(e) => map_parse_error(file_id, source_code, e),
        RoxError::Evaluate(e) => map_runtime_error(file_id, source_code, e),
        // 如果有其他错误（如 Readline），直接打印即可，不需要高亮源码
        _ => Diagnostic::error().with_message(format!("{}", error)),
    };

    // 3. 配置输出流
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = Config::default();

    // 4. 渲染输出
    if let Err(e) = term::emit_to_write_style(&mut writer.lock(), &config, &files, &diagnostic) {
        eprintln!("Error rendering diagnostic: {}", e);
        // 如果渲染挂了，回退到原始输出
        eprintln!("Original error: {}", error);
    }
}

// helper：行号 -> 字节范围
// codespan 需要字节位置 (Byte Index)，但当前实现 Token 只有行号 (Line Number)。
// 计算某一行的起始和结束字节位置。
fn line_range(source: &str, line_number: usize) -> Range<usize> {
    // 行号从 1 开始
    if line_number == 0 {
        return 0..0;
    }

    let mut current_line = 1;
    let mut line_start = 0;

    for (i, c) in source.char_indices() {
        if c == '\n' {
            if current_line == line_number {
                return line_start..i; // 返回这一行的范围
            }
            current_line += 1;
            line_start = i + 1;
        }
    }

    // 处理最后一行（可能没有换行符）
    if current_line == line_number {
        return line_start..source.len();
    }

    0..0 // 没找到该行
}

// 错误映射逻辑

fn map_scan_error(file_id: usize, source: &str, error: &ScanError) -> Diagnostic<usize> {
    // 假设 ScanError 包含一组错误，这里简单起见只取第一个
    if let Some(first) = error.0.first() {
        let (msg, line) = match first {
            SingleScanError::UnexpectedCharacter { c, line } => {
                (format!("Unexpected character '{}'", c), *line)
            }
            SingleScanError::UnterminatedString { line } => {
                ("Unterminated string".to_string(), *line)
            }
        };

        let range = line_range(source, line);

        Diagnostic::error()
            .with_message("Scanning Error")
            .with_labels(vec![Label::primary(file_id, range).with_message(msg)])
    } else {
        Diagnostic::error().with_message("Unknown Scanning Error")
    }
}

fn map_parse_error(file_id: usize, source: &str, error: &ParseError) -> Diagnostic<usize> {
    // 假设 ParseError 有 message 和 position/line
    // 根据 message 解析出行号，或者修改 ParseError 结构体直接携带行号
    // 暂时假设能解析出 "[line X]"
    let line = parse_line_from_msg(&error.message).unwrap_or(1);
    let range = line_range(source, line);

    Diagnostic::error()
        .with_message("Syntax Error")
        .with_labels(vec![
            Label::primary(file_id, range).with_message(&error.message),
        ])
}

fn map_runtime_error(_file_id: usize, _source: &str, error: &RuntimeError) -> Diagnostic<usize> {
    // 运行时错误目前可能没有行号信息，只能显示消息
    // TODO: RuntimeError 添加 token/line 字段
    Diagnostic::error()
        .with_message("Runtime Error")
        .with_notes(vec![format!("{}", error)])
}

// helper：从错误字符串提取行号
fn parse_line_from_msg(msg: &str) -> Option<usize> {
    if let Some(start) = msg.find("[line ")
        && let Some(end) = msg[start..].find(']')
    {
        let num_str = &msg[start + 6..start + end];
        return num_str.parse().ok();
    }

    None
}
