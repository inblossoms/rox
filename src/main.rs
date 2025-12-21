mod evaluate;
mod parser;
mod reader;
mod tokenizer;

type Read = reader::Error;
type Parse = parser::Error;
type Evaluate = evaluate::Error;
type Tokenize = tokenizer::Error;

#[derive(Debug)]
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

    match run() {
        Ok(out) => {
            println!("Success! {:?}", out);
        }
        Err(e) => {
            println!("Work goes wrong, failed info: {:?}", e);
        }
    }
}

fn run() -> Result<(), Error> {
    let source = reader::reader_source("file.lox")?;
    let tokens = tokenizer::tokenize(source)?;
    let ast = parser::parse(tokens)?;
    let out = evaluate::evaluate(ast)?;

    Ok(())
}
