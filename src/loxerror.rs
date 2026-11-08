use std::{fmt::Display, process::ExitCode};

use crate::Token;

#[derive(Debug, Clone, Copy)]
pub enum LoxErrorKind {
    ScanError,
    ParseError,
    RuntimeError,
}

#[derive(Debug, Clone)]
pub struct LoxError {
    pub kind: LoxErrorKind,
    message: String,
}

impl std::error::Error for LoxError {}

pub enum SourceLocation<'a> {
    Line(usize),
    Token(&'a Token<'a>),
}

impl LoxError {
    pub fn new_scan_error(message: &str, line: usize) -> Self {
        Self {
            kind: LoxErrorKind::ScanError,
            message: format!("[line {}] {}", line, message),
        }
    }

    pub fn new_parse_error(message: &str, source_location: SourceLocation) -> Self {
        let msg = match source_location {
            SourceLocation::Line(line) => &format!("[line {}] {}", line, message),
            SourceLocation::Token(t) => {
                &format!("[line {}] Error at '{}': {}", t.line, t.lexeme, message)
            }
        };

        Self {
            kind: LoxErrorKind::ParseError,
            message: msg.to_owned(),
        }
    }

    pub fn new_runtime_error(message: &str, source_location: SourceLocation) -> Self {
        let msg = match source_location {
            SourceLocation::Line(line) => format!("{}\n[line {}] ", message, line),
            SourceLocation::Token(t) => {
                format!("Error at '{}': {}\n[line {}] ", t.lexeme, message, t.line)
            }
        };

        Self {
            kind: LoxErrorKind::RuntimeError,
            message: msg,
        }
    }
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<LoxError> for ExitCode {
    fn from(err: LoxError) -> Self {
        match err.kind {
            LoxErrorKind::ScanError => ExitCode::from(65),
            LoxErrorKind::ParseError => ExitCode::from(65),
            LoxErrorKind::RuntimeError => ExitCode::from(65),
        }
    }
}
