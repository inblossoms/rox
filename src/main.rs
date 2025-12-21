mod evaluate;
mod parser;
mod reader;
mod tokenizer;

fn main() {
    println!("Hello, lox!");

    let source = reader::reader_source("file.lox").unwrap();
    let tokens = tokenizer::tokenize(source).unwrap();
    let ast = parser::parse(tokens).unwrap();
    let out = evaluate::evaluate(ast).unwrap();
}
