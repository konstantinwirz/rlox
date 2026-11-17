use std::fmt::Display;

use crate::{Expr, Token};

#[derive(Debug, Clone)]
pub enum Stmt<'a> {
    Expr(Expr<'a>),
    If {
        cond: Expr<'a>,
        then_branch: Box<Stmt<'a>>,
        else_branch: Option<Box<Stmt<'a>>>,
    },
    Print(Expr<'a>),
    Var {
        name: Token<'a>,
        initializer: Option<Expr<'a>>,
    },
    Block(Vec<Stmt<'a>>),
    While {
        cond: Expr<'a>,
        body: Box<Stmt<'a>>,
    },
}

impl<'a> Display for Stmt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Stmt::Expr(ref expr) => write!(f, "{expr}"),
            &Self::If {
                ref cond,
                ref then_branch,
                else_branch: None,
            } => write!(f, "if ({}) {{ {} }}", cond, then_branch),
            &Self::If {
                ref cond,
                ref then_branch,
                else_branch: Some(ref else_branch),
            } => write!(
                f,
                "if ({}) {{ {} }} {{ {} }}",
                cond, then_branch, else_branch
            ),
            &Stmt::Print(ref expr) => write!(f, "print {expr}"),
            &Stmt::Var {
                ref name,
                ref initializer,
            } => match initializer {
                Some(expr) => write!(f, "var {} = {expr}", name.lexeme),
                None => write!(f, "var {name}"),
            },
            &Stmt::Block(ref stmts) => {
                let mut output = String::new();
                output.push_str("{");
                for stmt in stmts {
                    output.push_str(&format!("\n\t{stmt}"));
                }
                output.push_str("\n}");
                f.write_str(&output)
            }
            &Self::While { ref cond, ref body } => {
                write!(f, "while {}\n{}", cond, body)
            }
        }
    }
}
