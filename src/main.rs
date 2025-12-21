mod evaluate;
mod parser;
mod reader;
mod tokenizer;

type Error = ();

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
    let source = reader::reader_source("file.lox").unwrap();
    let tokens = tokenizer::tokenize(source).unwrap();
    let ast = parser::parse(tokens).unwrap();
    let out = evaluate::evaluate(ast).unwrap();

    Ok(())
}
