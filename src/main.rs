mod evaluate;
mod parser;
mod reader;
mod tokenize;

fn main() {
    println!("Hello, lox!");

    reader::reader_source();
    tokenize::tokenize();
    parser::parse();
    evaluate::evaluate();
}
