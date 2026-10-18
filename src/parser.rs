use std::iter::Peekable;

use crate::{Expr, Lexer, LoxError, LoxValue, Token, TokenKind};

pub type ParseResult<'a> = Result<Expr<'a>, LoxError>;

pub struct Parser<'p> {
    lexer: Peekable<Lexer<'p>>,
}

impl<'p> Iterator for Parser<'p> {
    type Item = ParseResult<'p>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_end() {
            return None;
        }

        Some(self.parse())
    }
}

impl<'p> Parser<'p> {
    pub fn new(source: &'p str) -> Self {
        Self {
            lexer: Lexer::new(source).peekable(),
        }
    }

    fn parse(&mut self) -> ParseResult<'p> {
        self.parse_expr()
    }

    fn parse_equality(&mut self) -> ParseResult<'p> {
        let mut expr = self.parse_comparison()?;

        while let Some(op) = self.matches(&[TokenKind::BangEqual, TokenKind::EqualEqual])? {
            let right_expr = self.parse_comparison()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right_expr));
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> ParseResult<'p> {
        let mut expr = self.parse_term()?;

        while let Some(op) = self.matches(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ])? {
            let right_expr = self.parse_term()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right_expr));
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> ParseResult<'p> {
        let mut expr = self.parse_factor()?;

        while let Some(op) = self.matches(&[TokenKind::Minus, TokenKind::Plus])? {
            let right_expr = self.parse_factor()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right_expr))
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> ParseResult<'p> {
        let mut expr = self.parse_unary()?;

        while let Some(op) = self.matches(&[TokenKind::Slash, TokenKind::Star])? {
            let right_expr = self.parse_unary()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right_expr));
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> ParseResult<'p> {
        match self.matches(&[TokenKind::Bang, TokenKind::Minus])? {
            Some(op) => {
                let right_expr = self.parse_unary()?;
                Ok(Expr::Unary(op, Box::new(right_expr)))
            }
            None => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> ParseResult<'p> {
        if self.matches(&[TokenKind::False])?.is_some() {
            return Ok(Expr::Literal(LoxValue::Boolean(false)));
        }

        if self.matches(&[TokenKind::True])?.is_some() {
            return Ok(Expr::Literal(LoxValue::Boolean(true)));
        }

        if self.matches(&[TokenKind::Nil])?.is_some() {
            return Ok(Expr::Literal(LoxValue::Nil));
        }

        if let Some(token) = self.matches(&[TokenKind::Number, TokenKind::String])? {
            return Ok(Expr::Literal(token.literal));
        }

        if self.matches(&[TokenKind::LeftParen])?.is_some() {
            let expr = Box::new(self.parse_expr()?);
            self.consume(&TokenKind::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Group(expr));
        }

        Err(LoxError::new_parse_error(
            "Expect expression",
            self.peek()?.map_or_default(|t| t.line),
        ))
    }

    fn parse_expr(&mut self) -> ParseResult<'p> {
        self.parse_equality().map_err(|err| {
            _ = self.sync();
            err
        })
    }

    fn consume(&mut self, token_kind: &TokenKind, msg: &str) -> Result<(), LoxError> {
        if let Some(token) = self.peek()? {
            if token.kind == *token_kind {
                assert!(self.advance().is_ok());
                return Ok(());
            }

            // unexpected
            return Err(LoxError::new_parse_error(msg, token.line));
        }

        panic!("no more tokens");
    }

    fn sync(&mut self) -> Result<(), LoxError> {
        let token = self.advance()?;

        while !self.is_at_end() {
            if token.kind == TokenKind::Semicolon {
                return Ok(());
            }

            if let Some(token) = self.peek()? {
                if matches!(
                    token.kind,
                    TokenKind::Class
                        | TokenKind::Fun
                        | TokenKind::Var
                        | TokenKind::For
                        | TokenKind::If
                        | TokenKind::While
                        | TokenKind::Print
                        | TokenKind::Return
                ) {
                    return Ok(());
                }
            }

            assert!(self.advance().is_ok());
        }

        Ok(())
    }

    fn is_at_end(&mut self) -> bool {
        match self.peek() {
            Ok(Some(token)) if token.kind == TokenKind::Eof => true,
            Ok(None) => true,
            _ => false,
        }
    }

    fn check(&mut self, token_kind: &TokenKind) -> Result<Option<&Token<'p>>, LoxError> {
        match self.peek()? {
            Some(token) if &token.kind == token_kind => Ok(Some(token)),
            _ => Ok(None),
        }
    }

    fn peek(&mut self) -> Result<Option<&Token<'p>>, LoxError> {
        match self.lexer.peek() {
            Some(Ok(token)) => Ok(Some(token)),
            Some(Err(e)) => Err(e.clone()),
            _ => Ok(None),
        }
    }

    fn matches(&mut self, token_kinds: &[TokenKind]) -> Result<Option<Token<'p>>, LoxError> {
        for kind in token_kinds {
            if self.check(kind)?.is_some() {
                // found it
                return self.advance().map(Some);
            }
        }

        Ok(None)
    }

    /// advances the token iterator (scanner)
    /// make sure it's not at end, it will panic
    fn advance(&mut self) -> Result<Token<'p>, LoxError> {
        match self.lexer.next() {
            Some(Ok(token)) => Ok(token),
            Some(Err(err)) => Err(err.into()),
            _ => panic!("called advance on exhausted lexer"),
        }
    }
}
