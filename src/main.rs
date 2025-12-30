use std::{
    env,
    io::{Write, stdin, stdout},
};

use crate::{
    evaluate::{
        Interpreter, Value,
        interpreter::{self, RuntimeError},
    },
    reader::Source,
};

mod ast;
mod evaluate;
mod parser;
mod reader;
mod tokenizer;

type Read = reader::Error;
type Parse = parser::Error;
type Evaluate = interpreter::Error;
type Tokenize = tokenizer::Error;
type Runtime = RuntimeError;

#[derive(Debug)]
#[allow(dead_code)]
enum Error {
    Read(Read),
    Parse(Parse),
    Evaluate(Evaluate),
    Tokenize(Tokenize),
    Runtime(Runtime),
}

macro_rules! impl_from_error {
    ($enum_name:ident, $($variant:ident),+) => {
        $(
            impl From<$variant> for $enum_name {
                fn from(error: $variant) -> Self {
                    Self::$variant(error)
                }
            }
        )+
    };
}

impl_from_error!(Error, Read, Parse, Evaluate, Tokenize, Runtime);

fn main() {
    println!("Hello, rox!");

    //  ast::main();

    let input_args = env::args().collect::<Vec<_>>();

    if input_args.len() == 1 {
        run_prompt();
    } else if input_args.len() == 2 {
        match run_file(&input_args[1]) {
            Ok(_) => {
                println!("Success!");
            }
            Err(e) => {
                eprintln!("Work goes wrong, failed info: {:?}", e);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("Usage: rox [script]");
        std::process::exit(64);
    }
}

fn run_interpreter(source: Source) -> Result<Value, Error> {
    let tokens = tokenizer::tokenize(source)?;

    println!("Tokens: {:#?}", tokens);

    let ast = parser::parse(tokens)?;

    let mut interpreter = Interpreter::new();
    let out = interpreter.interpret(ast)?;

    Ok(out)
}

fn run_file(file: &str) -> Result<Value, Error> {
    let source = reader::reader_source(file)?;
    run_interpreter(source)
}

fn run_prompt() {
    let mut interpreter = Interpreter::new(); // 在循环外创建解释器实例

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");

        let source = reader::Source { contents: input };
        match run_interpreter_with_state(source, &mut interpreter) {
            // 使用带状态的函数
            Ok(r) => {
                if r != Value::Nil {
                    println!("{:?}", r);
                }
                println!("Press Ctrl+C to exit.")
            }
            Err(e) => eprintln!("Read line goes wrong, failed info: {:?}", e),
        }
    }
}

fn run_interpreter_with_state(
    source: Source,
    interpreter: &mut Interpreter,
) -> Result<Value, Error> {
    let tokens = tokenizer::tokenize(source)?;
    let ast = parser::parse(tokens)?;
    let out = interpreter.interpret(ast)?;

    Ok(out)
}
