use std::{
    env,
    io::{Write, stdin, stdout},
};

use crate::{
    error::RoxError,
    evaluate::{Interpreter, Value},
    reader::Source,
};

mod ast;
mod error;
mod evaluate;
mod parser;
mod reader;
mod tokenizer;

fn main() -> Result<(), RoxError> {
    println!("lox v0.1.0 - A simple scripting language interpreter");

    let input_args = env::args().collect::<Vec<_>>();
    let mut interpreter = Interpreter::new(); // 需要保证解释器在解析过程中的上下文一致性，所以将其提取到上层

    if input_args.len() == 1 {
        println!("Type 'help' for more information or press Ctrl+C to exit.");
        Ok(run_prompt(&mut interpreter))
    } else if input_args.len() == 2 {
        match run_file(&input_args[1], &mut interpreter) {
            Ok(r) => {
                // 如果执行成功且有返回值，显示结果
                if r != Value::Nil {
                    println!("Result: {}", r);
                } else {
                    println!("Script executed successfully.");
                }
                Ok(())
            }
            Err(e) => {
                eprintln!("Error executing script! {e}");
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
    run_interpreter_with_state(source, interpreter)
}

fn run_prompt(interpreter: &mut Interpreter) {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");

        let source = reader::Source { contents: input };
        // 由于 interpreter 通过外部传入，所以不需要担心命令行输入时解析出的 code 上下文不一致导致的不期望结果
        // 当 interpreter 在 loop 中被运行创建，那么每次运行都会创建一个新的上下文
        match run_interpreter_with_state(source, interpreter) {
            Ok(r) => {
                if r != Value::Nil {
                    println!("{:?}", r);
                }
            }
            Err(e) => eprintln!("Read line goes wrong, failed info: {:?}", e),
        }
    }
}

fn run_interpreter_with_state(
    source: Source,
    interpreter: &mut Interpreter,
) -> Result<Value, RoxError> {
    let tokens = tokenizer::tokenize(source)?;
    //  println!("Tokens: {:#?}", tokens);

    let ast = parser::parse(tokens)?;
    //  println!("AST: {:#?}", ast);

    let out = interpreter.interpret(ast)?;

    Ok(out)
}
