use std::fmt::Display;

use crate::{Expr, Token};

#[derive(Debug, Clone)]
pub enum Stmt<'a> {
    Expr(Expr<'a>),
    Print(Expr<'a>),
    Var {
        name: Token<'a>,
        initializer: Option<Expr<'a>>,
    },
    Block(Vec<Stmt<'a>>),
}

impl<'a> Display for Stmt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Stmt::Expr(ref expr) => write!(f, "{expr}"),
            &Stmt::Print(ref expr) => write!(f, "print {expr}"),
            &Stmt::Var {
                ref name,
                ref initializer,
            } => match initializer {
                Some(expr) => write!(f, "var {}={expr}", name.lexeme),
                None => write!(f, "var {name}"),
            },
            &Stmt::Block(ref stmts) => {
                let mut output = String::new();
                output.push_str("{");
                for stmt in stmts {
                    output.push_str(&format!("\n{stmt}"));
                }
                output.push_str("\n}");
                f.write_str(&output)
            }
        }
    }
}
