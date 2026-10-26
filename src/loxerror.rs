use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum LoxErrorKind {
    ScanError,
    ParseError,
    RuntimeError,
}

#[derive(Debug, Clone)]
pub struct LoxError {
    kind: LoxErrorKind,
    line: Option<usize>,
    message: String,
}

impl std::error::Error for LoxError {}

impl LoxError {
    pub fn new_scan_error(message: &str, line: usize) -> Self {
        Self {
            kind: LoxErrorKind::ScanError,
            line: Some(line),
            message: message.to_owned(),
        }
    }

    pub fn new_parse_error(message: &str, line: usize) -> Self {
        Self {
            kind: LoxErrorKind::ParseError,
            line: Some(line),
            message: message.to_owned(),
        }
    }

    pub fn new_runtime_error(messsage: &str) -> Self {
        Self {
            kind: LoxErrorKind::RuntimeError,
            line: None,
            message: messsage.to_owned(),
        }
    }
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.line {
            Some(line) => write!(f, "[line {}] {}", line, self.message),
            _ => write!(f, "{}", self.message),
        }
    }
}
