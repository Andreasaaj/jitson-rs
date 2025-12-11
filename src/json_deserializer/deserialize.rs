use std::collections::HashMap;

use crate::json_deserializer::{JsonValue, JsonNumber, JsonError};


pub fn parse_json(_input: &str) -> Result<JsonValue, JsonError> {
    // parser placeholder
    // TODO: Make parser
    
    let mut dummy_hashmap = HashMap::new();
    dummy_hashmap.insert("a".to_string(), JsonValue::Number(JsonNumber::Float(1.0)));
    let json_object = JsonValue::Object(dummy_hashmap);

    Ok(json_object)
    // Ok(JsonValue::Null)
}
