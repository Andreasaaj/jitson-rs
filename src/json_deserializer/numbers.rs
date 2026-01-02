#[derive(Debug, Clone, PartialEq)]
pub enum JsonNumber {
    Int(i64),
    Float(f64),
}

impl std::fmt::Display for JsonNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonNumber::Int(n) => write!(f, "{}", n),
            JsonNumber::Float(fl) => write!(f, "{}", fl),
        }
    }
}
