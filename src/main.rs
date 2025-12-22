use std::env;

use crate::reader::Source;

mod evaluate;
mod parser;
mod reader;
mod tokenizer;

type Read = reader::Error;
type Parse = parser::Error;
type Evaluate = evaluate::Error;
type Tokenize = tokenizer::Error;

#[derive(Debug)]
#[allow(dead_code)]
enum Error {
    Read(Read),
    Parse(Parse),
    Evaluate(Evaluate),
    Tokenize(Tokenize),
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

impl_from_error!(Error, Read, Parse, Evaluate, Tokenize);

fn main() {
    println!("Hello, lox!");

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
        eprintln!("Usage: lox [script]");
        std::process::exit(64);
    }
}

fn run_interpreter(source: Source) -> Result<(), Error> {
    let tokens = tokenizer::tokenize(source)?;
    let ast = parser::parse(tokens)?;
    let _out = evaluate::evaluate(ast)?;

    Ok(())
}

fn run_file(file: &str) -> Result<(), Error> {
    let source = reader::reader_source(file)?;
    run_interpreter(source)
}

fn run_prompt() {
    todo!()
}
