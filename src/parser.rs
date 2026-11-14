use std::iter::Peekable;

use crate::{Expr, Lexer, LoxError, LoxValue, Stmt, Token, TokenKind, loxerror::SourceLocation};

pub type ParseResult<'a> = Result<Option<Stmt<'a>>, LoxError>;

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
        if self.is_at_end() {
            return Ok(None);
        }

        match self.parse_decl() {
            Ok(stmt) => Ok(Some(stmt)),
            Err(err) => {
                _ = self.sync();
                Err(err)
            }
        }
    }

    fn parse_decl(&mut self) -> Result<Stmt<'p>, LoxError> {
        if self.matches(&[TokenKind::Var])?.is_some() {
            return self.parse_var_decl();
        }

        self.parse_stmt()
    }

    fn parse_stmt(&mut self) -> Result<Stmt<'p>, LoxError> {
        if self.matches(&[TokenKind::If])?.is_some() {
            return self.parse_if_stmt();
        }

        if self.matches(&[TokenKind::Print])?.is_some() {
            return self.parse_print_stmt();
        }

        if self.matches(&[TokenKind::LeftBrace])?.is_some() {
            return self.parse_block();
        }

        self.parse_expr_stmt()
    }

    fn parse_if_stmt(&mut self) -> Result<Stmt<'p>, LoxError> {
        _ = self.consume(&TokenKind::LeftParen, "Expect '(' after 'if'.");
        let cond = self.parse_expr()?;
        _ = self.consume(&TokenKind::RightParen, "Expect ')' after if condition.");

        let then_branch = Box::new(self.parse_stmt()?);
        let else_branch = if self.matches(&[TokenKind::Else])?.is_some() {
            Some(Box::new(self.parse_stmt()?))
        } else {
            None
        };

        Ok(Stmt::If {
            cond,
            then_branch,
            else_branch,
        })
    }

    fn parse_block(&mut self) -> Result<Stmt<'p>, LoxError> {
        let mut stmts: Vec<Stmt<'p>> = Vec::new();

        while !self.is_at_end() && self.check(&TokenKind::RightBrace)?.is_none() {
            stmts.push(self.parse_decl()?);
        }

        _ = self.consume(&TokenKind::RightBrace, "Expect '}' after block.")?;
        Ok(Stmt::Block(stmts))
    }

    pub fn parse_expr(&mut self) -> Result<Expr<'p>, LoxError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr<'p>, LoxError> {
        let expr = self.parse_or_expr()?;

        if let Some(equals) = self.matches(&[TokenKind::Equal])? {
            let value = self.parse_assignment()?;
            return match expr {
                Expr::Var(name) => Ok(Expr::Assign(name, Box::new(value))),
                _ => Err(LoxError::new_parse_error(
                    "Invalid assignment target.",
                    SourceLocation::Token(&equals),
                )),
            };
        }

        Ok(expr)
    }

    fn parse_or_expr(&mut self) -> Result<Expr<'p>, LoxError> {
        let mut expr = self.parse_and_expr()?;

        while let Some(op) = self.matches(&[TokenKind::Or])? {
            let right = self.parse_and_expr()?;
            expr = Expr::Logical(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_and_expr(&mut self) -> Result<Expr<'p>, LoxError> {
        let mut expr = self.parse_equality_expr()?;

        while let Some(op) = self.matches(&[TokenKind::And])? {
            let right = self.parse_equality_expr()?;
            expr = Expr::Logical(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_var_decl(&mut self) -> Result<Stmt<'p>, LoxError> {
        let name: Token<'_> = self.consume(&TokenKind::Identifier, "Expect variable name.")?;
        let initializer = if self.matches(&[TokenKind::Equal])?.is_some() {
            Some(self.parse_expr()?)
        } else {
            None
        };

        self.consume(
            &TokenKind::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::Var { name, initializer })
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt<'p>, LoxError> {
        let expr = self.parse_expr()?;
        _ = self.consume(&TokenKind::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expr(expr))
    }

    fn parse_print_stmt(&mut self) -> Result<Stmt<'p>, LoxError> {
        let expr = self.parse_expr()?;
        _ = self.consume(&TokenKind::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(expr))
    }

    fn parse_equality_expr(&mut self) -> Result<Expr<'p>, LoxError> {
        let mut expr = self.parse_comparison_expr()?;

        while let Some(op) = self.matches(&[TokenKind::BangEqual, TokenKind::EqualEqual])? {
            let right = self.parse_comparison_expr()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_comparison_expr(&mut self) -> Result<Expr<'p>, LoxError> {
        let mut expr = self.parse_term_expr()?;

        while let Some(op) = self.matches(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ])? {
            let right_expr = self.parse_term_expr()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right_expr));
        }

        Ok(expr)
    }

    fn parse_term_expr(&mut self) -> Result<Expr<'p>, LoxError> {
        let mut expr = self.parse_factor_expr()?;

        while let Some(op) = self.matches(&[TokenKind::Minus, TokenKind::Plus])? {
            let right_expr = self.parse_factor_expr()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right_expr))
        }

        Ok(expr)
    }

    fn parse_factor_expr(&mut self) -> Result<Expr<'p>, LoxError> {
        let mut expr = self.parse_unary_expr()?;

        while let Some(op) = self.matches(&[TokenKind::Slash, TokenKind::Star])? {
            let right_expr = self.parse_unary_expr()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right_expr));
        }

        Ok(expr)
    }

    fn parse_unary_expr(&mut self) -> Result<Expr<'p>, LoxError> {
        match self.matches(&[TokenKind::Bang, TokenKind::Minus])? {
            Some(op) => {
                let right_expr = self.parse_unary_expr()?;
                Ok(Expr::Unary(op, Box::new(right_expr)))
            }
            None => self.parse_primary_expr(),
        }
    }

    fn parse_primary_expr(&mut self) -> Result<Expr<'p>, LoxError> {
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

        if let Some(token) = self.matches(&[TokenKind::Identifier])? {
            return Ok(Expr::Var(token));
        }

        if self.matches(&[TokenKind::LeftParen])?.is_some() {
            let expr = Box::new(self.parse_expr()?);
            self.consume(&TokenKind::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Group(expr));
        }

        Err(LoxError::new_parse_error(
            "Expect expression.",
            SourceLocation::Token(
                self.peek()?
                    .expect("token must exist, we are not at end yet"),
            ),
        ))
    }

    fn consume(&mut self, token_kind: &TokenKind, error_msg: &str) -> Result<Token<'p>, LoxError> {
        if let Some(token) = self.peek()? {
            if token.kind == *token_kind {
                return Ok(self.advance()?);
            }

            // unexpected
            return Err(LoxError::new_parse_error(
                error_msg,
                SourceLocation::Token(token),
            ));
        }

        panic!("no more tokens, first check if we are at end")
    }

    fn sync(&mut self) -> Result<(), LoxError> {
        while !self.is_at_end() {
            let token = self.advance()?;
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
