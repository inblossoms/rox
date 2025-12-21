mod evaluate;
mod parser;
mod reader;
mod tokenizer;

fn main() {
    println!("Hello, lox!");

    let source = reader::reader_source("file.lox");
    let tokens = tokenizer::tokenize(source);
    let ast = parser::parse(tokens);
    let out = evaluate::evaluate(ast);
}
