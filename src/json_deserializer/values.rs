use std::collections::HashMap;
use std::fmt;
use crate::json_deserializer::numbers::JsonNumber;


#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(JsonNumber),
    Null,
    Bool(bool),
}


impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Bool(b) => write!(f, "{}", b),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::String(s) => write!(f, "\"{}\"", s),
            JsonValue::Array(arr) => {
                write!(f, "[")?;
                let mut first = true;
                for val in arr {
                    if !first { write!(f, ",")?; }
                    write!(f, "{}", val)?;
                    first = false;
                }
                write!(f, "]")
            }
            JsonValue::Object(obj) => {
                write!(f, "{{")?;
                let mut first = true;
                for (k, v) in obj {
                    if !first { write!(f, ",")?; }
                    write!(f, "\"{}\":{}", k, v)?;
                    first = false;
                }
                write!(f, "}}")
            }
        }
    }
}
