use formatjson::{
    tokenizer::tokenize,
    validator::{validate, ValidationError},
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
