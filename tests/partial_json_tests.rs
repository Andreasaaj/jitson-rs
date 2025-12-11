use jitson_rust::{json_deserializer::{self}, serde_parser};


#[test]
fn parsing_partial_json_succeeds() {
    let input = r#"{ "a" : 10,"#;
    let corrected_input = r#"{ "a" : 10 }"#;
    
    let from_custom = json_deserializer::deserialize::parse_json(input).unwrap();
    let from_serde = serde_parser::parse_json(corrected_input).unwrap();

    // Compare output to serde’s
    assert_eq!(from_custom.to_string(), from_serde.to_string());
}
