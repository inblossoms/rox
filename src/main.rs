use std::{
    env, fs,
    io::{self, Write},
    path::Path,
};

use crate::{
    error::RoxError,
    evaluate::{Interpreter, Value, error::RuntimeError},
    reader::Source,
    resolver::Resolver,
};

pub mod ast;
pub mod error;
pub mod evaluate;
pub mod parser;
pub mod reader;
pub mod resolver;
pub mod std_lib;
pub mod tokenizer;

fn main() -> Result<(), RoxError> {
    println!("rox v0.1.0 - A simple scripting language interpreter");

    let input_args = env::args().collect::<Vec<_>>();

    // 实例化解释器 (包含 Global Environment)
    // 在这里实例化是为了让 REPL 模式下可以保持变量状态
    let mut interpreter = Interpreter::default();

    if input_args.len() == 1 {
        println!("Type 'help' for more information or press Ctrl+C to exit.");
        run_prompt(&mut interpreter);
        Ok(())
    } else if input_args.len() == 2 {
        match run_file(&input_args[1], &mut interpreter) {
            Ok(r) => {
                // 如果是执行 script，在非显式 Print 的情况下通常不需要打印 Nil
                if r != Value::Nil {
                    // 只有当脚本最后产生了一个非 Nil 值时才打印
                    //   println!("Result: {}", r);
                }
                Ok(())
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("Usage: rox [script]");
        std::process::exit(64);
    }
}

fn run_file(file: &str, interpreter: &mut Interpreter) -> Result<Value, RoxError> {
    let source = reader::reader_source(file)?;

    // 设置初始路径上下文
    // 获取入口文件的"绝对路径的父目录"
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
        eprintln!(
            "Warning: Could not resolve absolute path for entry file. Falling back to current working directory."
        );
        if let Ok(cwd) = env::current_dir() {
            interpreter.path_stack.push(cwd);
        } else {
            return Err(RoxError::Evaluate(RuntimeError::Generic(
                "Could not determine path context".into(),
            )));
        }
    }

    let result = run_interpreter_with_state(source, interpreter);

    // 虽然程序将要退出，但依旧保持 Interpreter 状态干净（便于测试或多次调用）
    if !interpreter.path_stack.is_empty() {
        interpreter.path_stack.pop();
    }

    result
}

fn run_prompt(interpreter: &mut Interpreter) {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let source = reader::Source { contents: input };
                // REPL：如果出错，打印错误但不退出进程
                match run_interpreter_with_state(source, interpreter) {
                    Ok(r) => {
                        // REPL：打印表达式结果
                        if r != Value::Nil {
                            println!("{}", r);
                        }
                    }
                    Err(e) => eprintln!("{}", e),
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }
}

fn run_interpreter_with_state(
    source: Source,
    interpreter: &mut Interpreter,
) -> Result<Value, RoxError> {
    let tokens = tokenizer::tokenize(source)?;
    // println!("Tokens: {:#?}", tokens);

    let ast = parser::parse(tokens)?;
    // println!("AST: {:#?}", ast);

    // 语义分析 (Resolver)
    let mut resolver = Resolver::new(interpreter);

    if let Err(msg) = resolver.resolve_stmts(&ast.body) {
        // 将 Resolver 的 String 错误转换为 RoxError
        // 这里暂时借用 RuntimeError::Generic
        // TODO: 定义 ResolutionError
        return Err(RoxError::Evaluate(RuntimeError::Generic(format!(
            "Resolution Error: {}",
            msg
        ))));
    }

    let out = interpreter.interpret(ast)?;

    Ok(out)
}
