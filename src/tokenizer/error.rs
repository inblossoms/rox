use std::fmt;

#[derive(Debug, Clone)]
pub enum ScanError {
    UnexpectedCharacter { c: char, line: usize },
    UnterminatedString { line: usize },
}

#[derive(Debug)]
pub struct Error(pub Vec<ScanError>);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // self.0 访问 Tuple Struct 的第一个元素 (即 Vec<ScanError>)
        for (index, _error) in self.0.iter().enumerate() {
            if index > 0 {
                writeln!(f)?; // 如果不是第一个错误，先换行
            }
            // write!(f, "{}", error)?; // 委托给 ScanError 的 Display
        }
        Ok(())
    }
}
