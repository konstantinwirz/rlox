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
    line: usize,
    message: String,
}

impl LoxError {
    pub fn new_scan_error(message: &str, line: usize) -> Self {
        Self {
            kind: LoxErrorKind::ScanError,
            line,
            message: message.to_owned(),
        }
    }

    pub fn new_parse_error(message: &str, line: usize) -> Self {
        Self {
            kind: LoxErrorKind::ParseError,
            line,
            message: message.to_owned(),
        }
    }
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] {}", self.line, self.message)
    }
}
