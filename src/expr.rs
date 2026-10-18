use std::fmt::{Display, Formatter};

use crate::{LoxValue, Token};

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Binary(Box<Expr<'a>>, Token<'a>, Box<Expr<'a>>),
    Unary(Token<'a>, Box<Expr<'a>>),
    Literal(LoxValue<'a>),
    Group(Box<Expr<'a>>),
}

impl Display for Expr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            &Expr::Literal(ref v) => write!(f, "{v}"),
            &Expr::Group(ref expr) => write!(f, "(group {expr})"),
            &Expr::Unary(ref op, ref expr) => write!(f, "({} {})", op.lexeme, expr),
            &Expr::Binary(ref left, ref op, ref right) => {
                write!(f, "({} {} {})", op.lexeme, left, right)
            }
        }
    }
}
