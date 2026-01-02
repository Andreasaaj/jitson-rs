#[derive(Debug)]
pub enum JsonError {
    UnexpectedEOF,
    UnexpectedToken(String),
    InvalidNumber(String),
    InvalidString(String),
    Other(String),
}

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JsonError::UnexpectedEOF => write!(f, "Unexpected end of input"),
            JsonError::UnexpectedToken(tok) => write!(f, "Unexpected token: {}", tok),
            JsonError::InvalidNumber(num) => write!(f, "Invalid number: {}", num),
            JsonError::InvalidString(s) => write!(f, "Invalid string: {}", s),
            JsonError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for JsonError {}
