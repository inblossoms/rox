use crate::{
    diagnostics::print_diagnostic,
    error::RoxError,
    evaluate::{Interpreter, Value, error::RuntimeError},
    reader::Source,
    resolver::Resolver,
};
use rustyline::{DefaultEditor, error::ReadlineError};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

mod ast;
mod diagnostics;
mod error;
mod evaluate;
mod parser;
mod reader;
mod resolver;
mod std_lib;
mod tokenizer;

fn main() -> Result<(), RoxError> {
    println!("rox v0.1.0 - A simple scripting language interpreter");

    let input_args = env::args().collect::<Vec<_>>();

    // 实例化解释器 （包含 Global Environment）
    // 在这里实例化是为了让 REPL 模式下可以保持变量状态
    let mut interpreter = Interpreter::default();

    if input_args.len() == 1 {
        println!("Type 'help' for more information or press Ctrl+C to exit.");

        // REPL 模式错误处理
        if let Err(e) = run_prompt(&mut interpreter) {
            eprintln!("REPL Error: {}", e);
            std::process::exit(1);
        }
        Ok(())
    } else if input_args.len() == 2 {
        // 脚本模式错误处理
        match run_file(&input_args[1], &mut interpreter) {
            Ok(r) => {
                if r != Value::Nil {
                    // 脚本模式通常不打印返回值，除非显式 print
                }
                Ok(())
            }
            Err(e) => {
                // 读取文件内容用于报错高亮
                let source_code = fs::read_to_string(&input_args[1]).unwrap_or_default();
                print_diagnostic(&input_args[1], &source_code, &e);

                // 脚本错误非零退出
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("Usage: rox [script]");
        std::process::exit(64);
    }
}

fn run_file(file: &str, interpreter: &mut Interpreter) -> Result<Value, RoxError> {
    let source = crate::reader::reader_source(file)?;

    // 设置路径上下文 (用于 import)
    let entry_path = Path::new(file);
    // 解析相对路径为绝对路径 (canonicalize: "scripts/file.rox" -> "/Users/me/rox/scripts/file.rox")
    if let Ok(absolute_path) = fs::canonicalize(entry_path) {
        if let Some(parent_dir) = absolute_path.parent() {
            // 将入口文件所在的目录压入栈底
            // 这样，脚本内的第一次 import 就会基于这个目录，而不是 CWD
            interpreter.path_stack.push(parent_dir.to_path_buf());
        }
    } else {
        // 如果路径解析失败（因为 reader_source 已经读到文件，所以不大会失败）
        eprintln!("Warning: Could not resolve absolute path. Falling back to CWD.");
        if let Ok(cwd) = env::current_dir() {
            interpreter.path_stack.push(cwd);
        }
    }

    let result = run_interpreter_with_state(source, interpreter);

    if !interpreter.path_stack.is_empty() {
        interpreter.path_stack.pop();
    }

    result
}

fn run_prompt(interpreter: &mut Interpreter) -> Result<(), RoxError> {
    let mut rl = DefaultEditor::new()?;
    let history_file = get_history_path();

    // 加载历史记录
    if rl.load_history(&history_file).is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                // 处理空行
                if line.trim().is_empty() {
                    continue;
                }

                // 添加历史
                let _ = rl.add_history_entry(line.as_str());

                // 解释器会消耗 source，如果报错了，需要原始字符串传给 diagnostics
                let source_code = line.clone();
                let source = crate::reader::Source { contents: line };

                // 执行并捕获错误
                match run_interpreter_with_state(source, interpreter) {
                    Ok(r) => {
                        if r != Value::Nil {
                            println!("{}", r);
                        }
                    }
                    Err(e) => {
                        // e: RoxError，source_code: clone 的字符串
                        print_diagnostic("<stdin>", &source_code, &e);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            // 处理 Readline 自身的错误 (IO错误等)
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                return Err(RoxError::Readline(err));
            }
        }
    }

    // 保存历史
    if let Err(e) = rl.save_history(&history_file) {
        eprintln!("Warning: Failed to save history: {}", e);
    }

    Ok(())
}

fn run_interpreter_with_state(
    source: Source,
    interpreter: &mut Interpreter,
) -> Result<Value, RoxError> {
    // 1. Tokenize
    let tokens = crate::tokenizer::tokenize(source)?;

    // 2. Parse
    let ast = crate::parser::parse(tokens)?;

    // 3. Resolve
    let mut resolver = Resolver::new(interpreter);
    if let Err(msg) = resolver.resolve_stmts(&ast.body) {
        // 包装 Resolver 错误
        // 这里暂时借用 RuntimeError::Generic
        // TODO: 定义 ResolutionError
        return Err(RoxError::Evaluate(RuntimeError::Generic(format!(
            "Resolution Error: {}",
            msg
        ))));
    }

    // 4. Interpret
    let out = interpreter.interpret(ast)?;

    Ok(out)
}

fn get_history_path() -> PathBuf {
    // 依赖 'dirs' crate
    // 获取用户 Home 目录，eg. /Users/ray
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".rox_history");
    path
}
