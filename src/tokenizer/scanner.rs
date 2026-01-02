use super::{Literal, Token, TokenType, Tokens};
use crate::tokenizer::error::{Error as ScannerError, ScanError};

pub struct Scanner {
    // 这里将输入的源文本转换为Vec<char>。
    // char' Rust类型表示0到0x10FFFFF范围内的Unicode码位。
    // 在内部，char是32位的，这样做的主要原因是扫描自然地与字符一起工作。
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    errors: Vec<ScanError>,
}

impl Scanner {
    /// 创建一个新的词法分析器实例
    fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            errors: vec![],
        }
    }

    /// 处理扫描过程中遇到的错误
    fn handle_error(&mut self, error: ScanError) {
        match error {
            ScanError::UnexpectedCharacter { c, line } => {
                println!("[line {}] Unexpected character: {}", line, c);
            }
            ScanError::UnterminatedString { line } => {
                println!("[line {}] Unterminated string", line);
            }
        }

        self.errors.push(error);
    }

    /// 检查当前字符是否匹配预期字符，如果匹配则消耗该字符
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    /// 获取当前位置的字符，但不消耗它
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source[self.current]
    }

    /// 获取当前位置下一个字符，但不消耗它
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source[self.current + 1]
    }

    /// 处理空白字符，更新行号
    fn handle_whitespace(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
        }
    }

    /// 添加一个没有字面值的标记到标记列表
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Literal::None);
    }

    /// 获取当前词素内容，对于字符串字面值可以去除引号
    fn lexeme(&self, is_string_lexeme: bool) -> String {
        if is_string_lexeme {
            // 提取字符串内容（去掉首尾引号）
            // start 指向第一个引号，current 指向闭合引号的后面
            return self.source[self.start + 1..self.current - 1]
                .iter()
                .collect::<String>();
        }

        return self.source[self.start..self.current].iter().collect();
    }

    /// 添加一个带有字面值的标记到标记列表
    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        self.tokens.push(Token::new(
            token_type,
            self.lexeme(false),
            self.line,
            literal,
        ));
    }

    /// 扫描单个标记
    fn scan_token(&mut self) {
        // 先消耗字符，推进指针
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => {
                let token_type = if self.match_char('=') {
                    TokenType::MinusEqual
                } else {
                    TokenType::Minus
                };
                self.add_token(token_type)
            }
            '+' => {
                let token_type = if self.match_char('=') {
                    TokenType::PlusEqual
                } else {
                    TokenType::Plus
                };
                self.add_token(token_type)
            }
            ';' => self.add_token(TokenType::Semicolon),
            '*' => {
                let token_type = if self.match_char('=') {
                    TokenType::StarEqual
                } else {
                    TokenType::Star
                };

                self.add_token(token_type);
            }
            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual // !=
                } else {
                    TokenType::Bang // ！
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
            '^' => {
                //  let token_type = if self.match_char('=') {
                //      TokenType::CaretEqual
                //  } else {
                //      TokenType::Caret
                //  };
                self.add_token(TokenType::BitXor);
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
                } else if self.match_char('*') {
                    self.scan_block_comment();
                } else if self.match_char('=') {
                    self.add_token(TokenType::SlashEqual);
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '&' => {
                let toke_type = if self.match_char('&') {
                    TokenType::And
                } else {
                    TokenType::BitAnd
                };

                self.add_token(toke_type);
            }
            '|' => {
                let toke_type = if self.match_char('|') {
                    TokenType::Or
                } else {
                    TokenType::BitOr
                };
                self.add_token(toke_type);
            }
            ' ' | '\t' | '\r' | '\n' => {
                self.handle_whitespace(c);
            }
            // 字符串处理
            '"' => self.is_string(),
            _ => {
                if c.is_numeric() {
                    self.is_digit();
                } else if c.is_alphabetic() || c == '_' {
                    self.is_identifier();
                } else {
                    self.handle_error(ScanError::UnexpectedCharacter { c, line: self.line });
                }
            }
        }
    }

    /// 扫描字符串字面值
    fn is_string(&mut self) {
        // Note：进入此方法时，开头的 '"' 已经被 advance() 消耗了
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            // 进入该判断 意味着字符串未闭合 且不存在下一个字符
            self.handle_error(ScanError::UnterminatedString { line: self.line });
            return;
        }

        // 消耗闭合的 '"'
        self.advance();

        self.add_token_with_literal(TokenType::String, Literal::String(self.lexeme(true)));
    }

    /// 扫描数字字面值
    fn is_digit(&mut self) {
        // 只要 peek 依旧是数字就继续消耗
        while self.peek().is_numeric() {
            self.advance();
        }

        // 小数部分
        if self.peek() == '.' {
            self.advance(); // 消耗 '.'

            while self.peek_next().is_numeric() {
                // 如果后续存在数字，则继续消耗
                while self.peek().is_numeric() {
                    self.advance();
                }
            }
        }

        // 存在解析失败的情况很少见，因为已经检查了字符，但在 Rust 中 unwrap 需要谨慎
        let value = self.lexeme(false).parse::<f64>().unwrap_or(0.0);
        self.add_token_with_literal(TokenType::Number, Literal::Number(value));
    }

    /// 扫描标识符或关键字
    fn is_identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let token_type = match self.lexeme(false).as_str() {
            "and" => TokenType::BitAnd,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::BitOr,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            "continue" => TokenType::Continue,
            "break" => TokenType::Break,
            _ => TokenType::Identifier,
        };

        self.add_token(token_type);
    }

    /// 检查是否到达源码末尾
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// 扫描所有标记
    fn scan_tokens(&mut self) -> Result<Tokens, ScannerError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "", self.line, Literal::None));

        if self.errors.len() > 0 {
            return Err(ScannerError(self.errors.clone()));
        } else {
            return Ok(Tokens {
                tokens: self.tokens.clone(),
            });
        }
    }
    // 处理多行注释
    fn scan_block_comment(&mut self) {
        while !self.is_at_end() {
            if self.peek() == '*' && self.peek_next() == '/' {
                self.advance(); // 消耗 *
                self.advance(); // 消耗 /
                return;
            }
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        // 可选：如果到了文件末尾还没闭合，可以报错 "Unterminated comment"
        //   self.error_reporter.error(self.line, "Unterminated comment.");
    }

    /// 消耗一个字符并返回该字符
    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }
}

/// 将源码转换为标记序列
pub fn tokenize(source: crate::reader::Source) -> Result<Tokens, ScannerError> {
    let mut scanner = Scanner::new(&source.contents);
    let tokens = scanner.scan_tokens()?;

    Ok(tokens)
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tokenizer_tests;
