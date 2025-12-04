use serde_json::Value;

pub fn parse_json(input: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str(input)
}
