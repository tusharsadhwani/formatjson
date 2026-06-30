use formatjson::{
    tokenizer::tokenize,
    validator::{validate, ValidationError},
    FormatJsonError,
};

#[test]
fn incomplete_json() {
    let unclosed_array = r#"[1, [2, 3, 4], 5, "foo", [[[[null]]]]"#;
    let tokens = tokenize(unclosed_array, "<source>".into()).unwrap();
    assert!(matches!(
        validate(&tokens).expect_err("Expected validate to fail"),
        ValidationError::UnexpectedEOF(_)
    ));
}

#[test]
fn bad_syntax_json() {
    let unclosed_array = r#"{]"#;
    let tokens = tokenize(unclosed_array, "<source>".into()).unwrap();
    assert!(matches!(
        validate(&tokens).expect_err("Expected validate to fail"),
        ValidationError::UnexpectedToken(_, _)
    ));
}

#[test]
fn extra_data() {
    let extra_data = r#"[],[]"#;
    let tokens = tokenize(extra_data, "<source>".into()).unwrap();
    assert!(matches!(
        validate(&tokens).expect_err("Expected validate to fail"),
        ValidationError::UnexpectedToken(_, _)
    ));
}

#[test]
fn unescaped_newline_in_string() {
    // A literal (unescaped) newline inside a string is invalid JSON; it must
    // be written as "\n". See https://github.com/tusharsadhwani/formatjson/issues/4
    let bad = "{\n  \"test string\": \"\n  \"\n}";
    assert!(matches!(
        tokenize(bad, "<source>".into()).expect_err("Expected tokenize to fail"),
        FormatJsonError::InvalidSyntax(_)
    ));
}

#[test]
fn escaped_newline_in_string() {
    // The escaped form is valid.
    let good = r#"{"test string": "\n"}"#;
    let tokens = tokenize(good, "<source>".into()).unwrap();
    validate(&tokens).expect("Expected validate to pass");
}

#[test]
fn valid_json() {
    let unclosed_array = r#"{"foo": []}"#;
    let tokens = tokenize(unclosed_array, "<source>".into()).unwrap();
    validate(&tokens).expect("Expected validate to pass");
}

#[test]
fn valid_json_empty() {
    let unclosed_array = r#"{}"#;
    let tokens = tokenize(unclosed_array, "<source>".into()).unwrap();
    validate(&tokens).expect("Expected validate to pass");
}
