use std::fmt::{Display, Formatter};

use crate::{LoxValue, Token};

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Binary(Box<Expr<'a>>, Token<'a>, Box<Expr<'a>>),
    Unary(Token<'a>, Box<Expr<'a>>),
    Literal(LoxValue<'a>),
    Logical(
        /* left */ Box<Expr<'a>>,
        /* operator */ Token<'a>,
        /* right */ Box<Expr<'a>>,
    ),
    Group(Box<Expr<'a>>),
    Var(/* name */ Token<'a>),
    Assign(/* name */ Token<'a>, /* value */ Box<Expr<'a>>),
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
            &Expr::Var(ref name) => write!(f, "{}", name.lexeme),
            &Expr::Assign(ref name, ref value) => write!(f, "{} = {}", name.lexeme, value),
            &Self::Logical(ref left, ref op, ref right) => {
                write!(f, "{} {} {}", left, op.lexeme, right)
            }
        }
    }
}
