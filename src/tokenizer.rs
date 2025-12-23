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
    pub fn new(
        token_type: TokenType,
        lexeme: impl Into<String>,
        line: usize,
        literal: Literal,
    ) -> Self {
        Self {
            token_type,
            lexeme: lexeme.into(),
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

    fn consume(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source[self.current + 1]
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Literal::None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        let text = self.source[self.start..self.current]
            .iter()
            .collect::<String>();
        self.tokens
            .push(Token::new(token_type, text, self.line, literal));
    }

    fn scan_token(&mut self) {
        match self.peek() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_char('/') {
                    // 跳过注释 - 继续读取直到行尾
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\t' | '\r' => {
                self.skip_whitespace();
            }
            '\n' => {
                self.line += 1;
                self.advance();
            }
            _ => {
                // 处理标识符和关键字
                if self.peek().is_alphabetic() {
                    while self.peek_next().is_alphanumeric() {
                        self.advance();
                    }
                    // 检查当前字符是否还是标识符的一部分
                    if self.peek().is_alphanumeric() {
                        self.advance();
                    }
                    let text: String = self.source[self.start..self.current].iter().collect();
                    // 检查是否为关键字
                    let token_type = match text.as_str() {
                        "and" => TokenType::And,
                        "class" => TokenType::Class,
                        "else" => TokenType::Else,
                        "false" => TokenType::False,
                        "fun" => TokenType::Fun,
                        "for" => TokenType::For,
                        "if" => TokenType::If,
                        "nil" => TokenType::Nil,
                        "or" => TokenType::Or,
                        "print" => TokenType::Print,
                        "return" => TokenType::Return,
                        "super" => TokenType::Super,
                        "this" => TokenType::This,
                        "true" => TokenType::True,
                        "var" => TokenType::Var,
                        "while" => TokenType::While,
                        _ => TokenType::Identifier,
                    };
                    self.add_token(token_type);
                } else if self.peek().is_numeric() {
                    while self.peek_next().is_numeric() {
                        self.advance();
                    }
                    // 检查当前字符是否还是数字的一部分
                    if self.peek().is_numeric() {
                        self.advance();
                    }
                    let text: String = self.source[self.start..self.current].iter().collect();
                    if let Ok(number) = text.parse::<f64>() {
                        self.add_token_with_literal(TokenType::Number, Literal::Number(number));
                    } else {
                        self.add_token(TokenType::Number);
                    }
                } else {
                    // 处理字符串字面量
                    if self.peek() == '"' {
                        self.advance(); // 跳过开始引号
                        while self.peek() != '"' && !self.is_at_end() {
                            if self.peek() == '\n' {
                                self.line += 1;
                            }
                            self.advance();
                        }

                        if self.is_at_end() {
                            // 错误：未闭合的字符串
                            // 为了防止死循环，我们需要确保位置被推进
                            // 可以选择跳过未闭合的字符串或报告错误
                            // 这里我们简单地推进位置
                        } else {
                            self.advance(); // 跳过结束引号
                            let text: String = self.source[self.start + 1..self.current - 1]
                                .iter()
                                .collect();
                            self.add_token_with_literal(TokenType::String, Literal::String(text));
                        }
                    } else {
                        // 遇到无法识别的字符，跳过它以避免死循环
                        self.advance();
                    }
                }
            }
        }
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

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }
}

pub fn tokenize(source: Source) -> Result<Tokens, Error> {
    let mut scanner = Scanner::new(&source.content);
    let tokens = scanner.scan_tokens();

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!((), ());
    }
}
