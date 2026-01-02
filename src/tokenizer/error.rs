#[derive(Debug, Clone)]
pub enum ScanError {
    UnexpectedCharacter { c: char, line: usize },
    UnterminatedString { line: usize },
}

#[derive(Debug)]
pub struct Error(pub Vec<ScanError>);
