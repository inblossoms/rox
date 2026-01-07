use crate::parser::tests::{assert_parse, parse_to_string};

#[test]
fn test_empty_program() {
    let result = parse_to_string("");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_extra_semicolons() {
    assert_parse(";;;", ";\n;\n;");
}
