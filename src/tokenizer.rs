use crate::reader::Source;

#[derive(Debug, Clone, PartialEq)]
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

    // whitespace
    Space,
    Tab,
    CarriageReturn,
    Newline,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    None,
}

#[derive(Debug, Clone, PartialEq)]
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

    fn add_whitespace_token(&mut self, c: char) {
        match c {
            ' ' => self.add_token(TokenType::Space),
            '\t' => self.add_token(TokenType::Tab),
            '\r' => self.add_token(TokenType::CarriageReturn),
            '\n' => {
                self.line += 1; // 先增加行号
                self.add_token(TokenType::Newline);
            }
            _ => {} // 不处理非空白字符
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
        // 核心修改：先消耗字符，推动指针前进
        let c = self.advance();

        match c {
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
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual // !=
                } else {
                    TokenType::Bang //  ！
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            }
            '/' => {
                if self.match_char('/') {
                    // 跳过注释
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\t' | '\r' | '\n' => {
                self.add_whitespace_token(c);
            }
            // 字符串处理
            '"' => self.handle_string(),
            _ => {
                if c.is_numeric() {
                    self.handle_number();
                } else if c.is_alphabetic() || c == '_' {
                    self.handle_identifier();
                } else {
                    // 遇到未知字符，Lox通常会报错，这里暂且忽略或打印错误
                    // eprintln!("Unexpected character: {}", c);
                }
            }
        }
    }

    fn handle_string(&mut self) {
        // 注意：进入此方法时，开头的 '"' 已经被 advance() 消耗了
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            // 错误：未闭合字符串
            return;
        }

        // 消耗闭合的 '"'
        self.advance();

        // 提取内容（去掉首尾引号）
        // start 指向第一个引号，current 指向闭合引号的后面
        let value = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect::<String>();
        self.add_token_with_literal(TokenType::String, Literal::String(value));
    }

    fn handle_number(&mut self) {
        // 这里的逻辑：只要 peek 是数字就继续消耗
        while self.peek().is_numeric() {
            self.advance();
        }

        // 处理小数部分
        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance(); // 消耗 '.'

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        // 解析失败的情况很少见，因为我们已经检查了字符，但在 Rust 中 unwrap 需要谨慎
        let value = text.parse::<f64>().unwrap_or(0.0);
        self.add_token_with_literal(TokenType::Number, Literal::Number(value));
    }

    fn handle_identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        let token_type = match text.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
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

    #[test]
    fn single_character() {
        let mut scanner = Scanner::new("(){};,.-+*");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.tokens,
            vec![
                Token::new(TokenType::LeftParen, "(", 1, Literal::None),
                Token::new(TokenType::RightParen, ")", 1, Literal::None),
                Token::new(TokenType::LeftBrace, "{", 1, Literal::None),
                Token::new(TokenType::RightBrace, "}", 1, Literal::None),
                Token::new(TokenType::Semicolon, ";", 1, Literal::None),
                Token::new(TokenType::Comma, ",", 1, Literal::None),
                Token::new(TokenType::Dot, ".", 1, Literal::None),
                Token::new(TokenType::Minus, "-", 1, Literal::None),
                Token::new(TokenType::Plus, "+", 1, Literal::None),
                Token::new(TokenType::Star, "*", 1, Literal::None),
                Token::new(TokenType::Eof, "", 1, Literal::None),
            ]
        )
    }

    #[test]
    fn two_character() {
        let mut scanner = Scanner::new("==<>!===<<=>>");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.tokens,
            vec![
                Token::new(TokenType::EqualEqual, "==", 1, Literal::None),
                Token::new(TokenType::Less, "<", 1, Literal::None),
                Token::new(TokenType::Greater, ">", 1, Literal::None),
                Token::new(TokenType::BangEqual, "!=", 1, Literal::None),
                Token::new(TokenType::EqualEqual, "==", 1, Literal::None),
                Token::new(TokenType::Less, "<", 1, Literal::None),
                Token::new(TokenType::LessEqual, "<=", 1, Literal::None),
                Token::new(TokenType::Greater, ">", 1, Literal::None),
                Token::new(TokenType::Greater, ">", 1, Literal::None),
                Token::new(TokenType::Eof, "", 1, Literal::None)
            ]
        )
    }

    #[test]
    fn keywords() {
        let mut scanner = Scanner::new(
            "and class else false for fun if nil or print return super this true var while",
        );
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.tokens,
            vec![
                Token::new(TokenType::And, "and", 1, Literal::None),
                Token::new(TokenType::Space, " ", 1, Literal::None),
                Token::new(TokenType::Class, "class", 1, Literal::None),
                Token::new(TokenType::Space, " ", 1, Literal::None),
                Token::new(TokenType::Else, "else", 1, Literal::None),
                Token::new(TokenType::Space, " ", 1, Literal::None),
                Token::new(TokenType::False, "false", 1, Literal::None),
                Token::new(TokenType::Space, " ", 1, Literal::None),
                Token::new(TokenType::For, "for", 1, Literal::None),
                Token::new(TokenType::Space, " ", 1, Literal::None),
                Token::new(TokenType::Fun, "fun", 1, Literal::None),
                Token::new(TokenType::Space, " ", 1, Literal::None),
                Token::new(TokenType::If, "if", 1, Literal::None),
                Token::new(TokenType::Space, " ", 1, Literal::None),
                Token::new(TokenType::Nil, "nil", 1, Literal::None),
                Token::new(TokenType::Space, " ", 1, Literal::None),
                Token::new(TokenType::Or, "or", 1, Literal::None),
                Token::new(TokenType::Space, " ", 1, Literal::None),
                Token::new(TokenType::Print, "print", 1, Literal::None),
                Token::new(TokenType::Space, " ", 1, Literal::None),
                Token::new(TokenType::Return, "return", 1, Literal::None),
                Token::new(TokenType::Space, " ", 1, Literal::None),
                Token::new(TokenType::Super, "super", 1, Literal::None),
                Token::new(TokenType::Space, " ", 1, Literal::None),
                Token::new(TokenType::This, "this", 1, Literal::None),
                Token::new(TokenType::Space, " ", 1, Literal::None),
                Token::new(TokenType::True, "true", 1, Literal::None),
                Token::new(TokenType::Space, " ", 1, Literal::None),
                Token::new(TokenType::Var, "var", 1, Literal::None),
                Token::new(TokenType::Space, " ", 1, Literal::None),
                Token::new(TokenType::While, "while", 1, Literal::None),
                Token::new(TokenType::Eof, "", 1, Literal::None),
            ]
        )
    }

    #[test]
    fn literals() {
        let mut scanner = Scanner::new("\"abc\"123.456");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.tokens,
            vec![
                Token::new(
                    TokenType::String,
                    "\"abc\"",
                    1,
                    Literal::String("abc".to_string())
                ),
                Token::new(TokenType::Number, "123.456", 1, Literal::Number(123.456)),
                Token::new(TokenType::Eof, "", 1, Literal::None),
            ]
        )
    }

    #[test]
    fn whitespace() {
        let mut scanner = Scanner::new(" \t\n\r");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.tokens,
            vec![
                Token::new(TokenType::Space, " ", 1, Literal::None),
                Token::new(TokenType::Tab, "\t", 1, Literal::None),
                Token::new(TokenType::Newline, "\n", 2, Literal::None),
                Token::new(TokenType::CarriageReturn, "\r", 2, Literal::None),
                Token::new(TokenType::Eof, "", 2, Literal::None),
            ]
        )
    }
}
