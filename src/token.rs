use std::fmt::Display;

use crate::LoxValue;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl TokenKind {
    pub(crate) fn new_identifier_or_kewyword(s: &str) -> TokenKind {
        match s {
            "and" => TokenKind::And,
            "class" => TokenKind::Class,
            "else" => TokenKind::Else,
            "false" => TokenKind::False,
            "fun" => TokenKind::Fun,
            "for" => TokenKind::For,
            "if" => TokenKind::If,
            "nil" => TokenKind::Nil,
            "or" => TokenKind::Or,
            "print" => TokenKind::Print,
            "return" => TokenKind::Return,
            "super" => TokenKind::Super,
            "this" => TokenKind::This,
            "true" => TokenKind::True,
            "var" => TokenKind::Var,
            "while" => TokenKind::While,
            _ => TokenKind::Identifier,
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            TokenKind::LeftParen => "LEFT_PAREN",
            TokenKind::RightParen => "RIGHT_PAREN",
            TokenKind::LeftBrace => "LEFT_BRACE",
            TokenKind::RightBrace => "RIGHT_BRACE",
            TokenKind::Comma => "COMMA",
            TokenKind::Dot => "DOT",
            TokenKind::Minus => "MINUS",
            TokenKind::Plus => "PLUS",
            TokenKind::Semicolon => "SEMICOLON",
            TokenKind::Slash => "SLASH",
            TokenKind::Star => "STAR",
            TokenKind::Bang => "BANG",
            TokenKind::BangEqual => "BANG_EQUAL",
            TokenKind::Equal => "EQUAL",
            TokenKind::EqualEqual => "EQUAL_EQUAL",
            TokenKind::Greater => "GREATER",
            TokenKind::GreaterEqual => "GREATER_EQUAL",
            TokenKind::Less => "LESS",
            TokenKind::LessEqual => "LESS_EQUAL",
            TokenKind::Identifier => "IDENTIFIER",
            TokenKind::String => "STRING",
            TokenKind::Number => "NUMBER",
            TokenKind::And => "AND",
            TokenKind::Class => "CLASS",
            TokenKind::Else => "ELSE",
            TokenKind::False => "FALSE",
            TokenKind::Fun => "FUN",
            TokenKind::For => "FOR",
            TokenKind::If => "IF",
            TokenKind::Nil => "NIL",
            TokenKind::Or => "OR",
            TokenKind::Print => "PRINT",
            TokenKind::Return => "RETURN",
            TokenKind::Super => "SUPER",
            TokenKind::This => "THIS",
            TokenKind::True => "TRUE",
            TokenKind::Var => "VAR",
            TokenKind::While => "WHILE",
            TokenKind::Eof => "EOF",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: &'a str,
    pub literal: LoxValue<'a>,
    pub line: usize,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.kind, self.lexeme, self.literal)
    }
}
