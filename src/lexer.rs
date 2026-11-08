use std::borrow::Cow;

use crate::{LoxError, LoxValue, Token, TokenKind};

pub struct Lexer<'s> {
    current: usize,
    start: usize,
    line: usize,
    source: &'s str,
    rest: &'s str,
    eof_emitted: bool,
}

impl<'s> Iterator for Lexer<'s> {
    type Item = ScanResult<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof_emitted {
            return None;
        }

        match self.scan_next_token() {
            Ok(token) if token.kind == TokenKind::Eof => {
                self.eof_emitted = true;
                Some(Ok(token))
            }
            result => Some(result),
        }
    }
}

pub type ScanResult<'s> = Result<Token<'s>, LoxError>;

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            current: 0,
            start: 0,
            line: 1,
            source,
            rest: source,
            eof_emitted: false,
        }
    }

    pub fn scan_next_token(&mut self) -> ScanResult<'s> {
        loop {
            if self.is_at_end() {
                break Ok(self.new_eof_token());
            }

            self.start = self.current;
            let ch = self.advance();
            if ch.is_ascii_whitespace() {
                if ch == '\n' {
                    self.line += 1;
                }
                continue;
            }

            break match ch {
                '(' => Ok(self.create_token(TokenKind::LeftParen)),
                ')' => Ok(self.create_token(TokenKind::RightParen)),
                '{' => Ok(self.create_token(TokenKind::LeftBrace)),
                '}' => Ok(self.create_token(TokenKind::RightBrace)),
                '+' => Ok(self.create_token(TokenKind::Plus)),
                '-' => Ok(self.create_token(TokenKind::Minus)),
                '*' => Ok(self.create_token(TokenKind::Star)),
                '.' => Ok(self.create_token(TokenKind::Dot)),
                ',' => Ok(self.create_token(TokenKind::Comma)),
                ';' => Ok(self.create_token(TokenKind::Semicolon)),
                '"' => self.scan_string(),
                '0'..='9' => self.scan_number(),
                'a'..='z' | 'A'..='Z' | '_' => self.scan_ident(),
                '=' => {
                    Ok(self.create_token_if_matches('=', TokenKind::EqualEqual, TokenKind::Equal))
                }
                '!' => Ok(self.create_token_if_matches('=', TokenKind::BangEqual, TokenKind::Bang)),
                '<' => Ok(self.create_token_if_matches('=', TokenKind::LessEqual, TokenKind::Less)),
                '>' => Ok(self.create_token_if_matches(
                    '=',
                    TokenKind::GreaterEqual,
                    TokenKind::Greater,
                )),
                '/' => {
                    if self.peek() == Some('/') {
                        while self.peek() != Some('\n') && !self.is_at_end() {
                            self.advance();
                        }
                        continue;
                    } else {
                        Ok(self.create_token(TokenKind::Slash))
                    }
                }
                c => Err(LoxError::new_scan_error(
                    &format!("Unexpected character: {}", c),
                    self.line,
                )),
            };
        }
    }

    fn create_token_if_matches(
        &mut self,
        expected: char,
        token_kind_if_matches: TokenKind,
        token_kind_if_doesnt_match: TokenKind,
    ) -> Token<'s> {
        let token_kind = if self.matches(expected) {
            token_kind_if_matches
        } else {
            token_kind_if_doesnt_match
        };
        self.create_token(token_kind)
    }

    fn scan_string(&mut self) -> ScanResult<'s> {
        loop {
            if self.peek() == Some('"') {
                assert_eq!(self.advance(), '"');
                let lexeme = &self.source[self.start..self.current];
                let literal = &self.source[self.start + 1..self.current - 1];

                return Ok(Token {
                    kind: TokenKind::String,
                    lexeme,
                    literal: LoxValue::String(Cow::from(literal)),
                    line: self.line,
                });
            }

            if self.is_at_end() {
                return Err(LoxError::new_scan_error(
                    "Error: Unterminated string.",
                    self.line,
                ));
            }

            if self.peek() == Some('\n') {
                self.line += 1;
            }

            self.advance();
        }
    }

    fn scan_number(&mut self) -> ScanResult<'s> {
        // scan al digits
        while let Some(d) = self.peek() {
            if !d.is_ascii_digit() {
                break;
            }
            assert!(self.advance().is_ascii_digit());
        }

        if self.peek() == Some('.') && self.peek_next().map_or(false, |c| c.is_ascii_digit()) {
            // consume .
            assert_eq!(self.advance(), '.');
            // consume digits
            while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                assert!(self.advance().is_ascii_digit());
            }
        }

        let lexeme = &self.source[self.start..self.current];

        let value: f64 = match lexeme.parse::<f64>() {
            Ok(v) => v,
            Err(_) => Err(LoxError::new_scan_error(
                &format!("Invalid number: {}", lexeme),
                self.line,
            ))?,
        };

        Ok(Token {
            kind: TokenKind::Number,
            lexeme,
            literal: LoxValue::Number(value),
            line: self.line,
        })
    }

    fn scan_ident(&mut self) -> ScanResult<'s> {
        while let Some(c) = self.peek() {
            if self.is_at_end() || !matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9') {
                break;
            }

            assert_eq!(self.advance(), c);
        }

        let lexeme: &str = &self.source[self.start..self.current];
        Ok(Token {
            kind: TokenKind::new_identifier_or_kewyword(lexeme),
            lexeme,
            literal: LoxValue::Nil,
            line: self.line,
        })
    }

    fn create_token(&self, kind: TokenKind) -> Token<'s> {
        Token {
            kind,
            lexeme: &self.source[self.start..self.current],
            literal: LoxValue::Nil,
            line: self.line,
        }
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn advance(&mut self) -> char {
        if self.is_at_end() {
            panic!("called advance on exhausted source");
        }

        let ch = self.rest.chars().next().expect("must exist");
        self.current += ch.len_utf8();
        self.rest = &self.source[self.current..];
        ch
    }

    fn peek(&self) -> Option<char> {
        self.rest.chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        self.rest.chars().nth(1)
    }

    fn matches(&mut self, c: char) -> bool {
        if self.peek() == Some(c) {
            assert_eq!(c, self.advance());
            true
        } else {
            false
        }
    }

    fn new_eof_token(&self) -> Token<'s> {
        Token {
            kind: TokenKind::Eof,
            lexeme: "",
            literal: LoxValue::Nil,
            line: self.line,
        }
    }
}
