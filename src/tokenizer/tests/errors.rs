use crate::tokenizer::scanner::Scanner;

#[test]
fn unexpected_character() {
    let mut scanner = Scanner::new("@");
    let result = scanner.scan_tokens();
    assert!(result.is_err());
}

#[test]
fn multiple_unexpected_characters() {
    let mut scanner = Scanner::new("@#$%");
    let result = scanner.scan_tokens();
    assert!(result.is_err());
}

#[test]
fn unterminated_string_error() {
    let mut scanner = Scanner::new(r#""Unterminated string"#);
    let result = scanner.scan_tokens();
    assert!(result.is_err());
}

#[test]
fn mixed_valid_and_invalid_input() {
    let mut scanner = Scanner::new("var x = 10; @ invalid");
    let result = scanner.scan_tokens();
    assert!(result.is_err());
}
