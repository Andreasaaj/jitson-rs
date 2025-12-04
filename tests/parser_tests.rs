use jitson_rust::{jitson_parser, serde_parser, JsonValue, JsonError, JsonNumber};
use std::collections::HashMap;


#[test]
fn compare_with_serde_simple() {
    let input = r#"{ "a": 1, "b": [true, false] }"#;

    let from_serde = serde_parser::parse_json(input).unwrap();
    let from_jitson = jitson_parser::parse_json(input).unwrap();

    // Compare output to serde’s
    assert_eq!(from_jitson.to_string(), from_serde.to_string());
}

#[test]
fn compare_with_manual_json_simple() {
    let input = r#"{ "a": 1, "b": [true, false] }"#;
    let from_jitson = jitson_parser::parse_json(input).unwrap();

    let inner_array = JsonValue::Array(vec![
        JsonValue::Bool(true),
        JsonValue::Bool(false),
    ]);

    let mut outer_hashmap = HashMap::new();
    outer_hashmap.insert(
        "a".to_string(),
        JsonValue::Number(JsonNumber::Int(1)),
    );
    outer_hashmap.insert(
        "b".to_string(),
        inner_array,
    );

    let json_value = JsonValue::Object(outer_hashmap);

    assert_eq!(from_jitson.to_string(), json_value.to_string());
}

#[test]
fn invalid_json_returns_any_json_error() {
    let input = "{ unquoted_key: 10 }";
    
    assert!(jitson_parser::parse_json(input).is_err());
}

#[test]
fn invalid_json_returns_error() {
    let input = "{ unquoted_key: 10 }";
    let err = jitson_parser::parse_json(input).unwrap_err();

    assert!(matches!(err, JsonError::UnexpectedToken(_)));
}
