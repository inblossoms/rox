use crate::reader::Source;

#[derive(Debug, Clone)]
pub enum TokenType {
    // single character
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    String,
    Number,

    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    None,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub literal: Literal,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, line: usize, literal: Literal) -> Self {
        Self {
            token_type,
            lexeme: String::from(lexeme),
            line,
            literal,
        }
    }
}

#[derive(Debug)]
pub struct Tokens {
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct Error {}

pub struct Scanner {
    // 这里将输入的源文本转换为Vec<char>。
    // char' Rust类型表示0到0x10FFFFF范围内的Unicode码位。
    // 在内部，char是32位的，这样做的主要原因是扫描自然地与字符一起工作。
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_token(&mut self) -> TokenType {
        todo!()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_tokens(&mut self) -> Tokens {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "", self.line, Literal::None));

        Tokens {
            tokens: self.tokens.clone(),
        }
    }
}

pub fn tokenize(_source: Source) -> Result<Tokens, Error> {
    println!("Tokenizing...");
    let tokens = vec![];

    Ok(Tokens { tokens })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!((), ());
    }
}
