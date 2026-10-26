use crate::{Expr, LoxError, LoxValue, Parser, Token, TokenKind};
use std::{borrow::Cow, iter::Peekable};

pub struct Interpreter<'i> {
    parser: Peekable<Parser<'i>>,
}

impl<'i> Interpreter<'i> {
    pub fn new(source: &'i str) -> Self {
        Self {
            parser: Parser::new(source).peekable(),
        }
    }

    pub fn eval(&mut self) -> Result<LoxValue<'i>, LoxError> {
        match self.parser.next().expect("expect at least an expression") {
            Ok(expr) => self.eval_expr(&expr),
            Err(_) => todo!(),
        }
    }

    pub fn print(value: &LoxValue<'_>) {
        let s: &str = match value {
            LoxValue::Nil => "nil",
            LoxValue::String(s) => s,
            LoxValue::Number(n) => &format!("{n}"),
            LoxValue::Boolean(b) => &format!("{b}"),
        };

        println!("{}", s);
    }

    fn eval_expr(&mut self, expr: &Expr<'i>) -> Result<LoxValue<'i>, LoxError> {
        match expr {
            &Expr::Binary(ref left, ref op, ref right) => self.eval_binary_expr(left, op, right),
            &Expr::Unary(ref op, ref expr) => self.eval_unary_expr(op, expr),
            &Expr::Literal(ref value) => Ok(value.clone()), // clone is cheap
            &Expr::Group(ref expr) => self.eval_expr(expr),
        }
    }

    fn eval_unary_expr(
        &mut self,
        op: &Token<'_>,
        expr: &Expr<'i>,
    ) -> Result<LoxValue<'i>, LoxError> {
        let value = self.eval_expr(expr)?;

        match &op.kind {
            &TokenKind::Minus => Ok(LoxValue::Number(-value.expect_number()?)),
            &TokenKind::Bang => Ok(LoxValue::Boolean(!value.is_truthy())),
            _ => Err(LoxError::new_runtime_error("invalid unary expression")),
        }
    }

    fn eval_binary_expr(
        &mut self,
        left: &Expr<'i>,
        op: &Token<'_>,
        right: &Expr<'i>,
    ) -> Result<LoxValue<'i>, LoxError> {
        let left_value = self.eval_expr(left)?;
        let right_value = self.eval_expr(right)?;

        match &op.kind {
            &TokenKind::Minus => Ok(LoxValue::Number(
                left_value.expect_number()? - right_value.expect_number()?,
            )),
            &TokenKind::Plus => match (&left_value, &right_value) {
                (&LoxValue::Number(ref l), &LoxValue::Number(ref r)) => Ok(LoxValue::Number(l + r)),
                (&LoxValue::String(ref l), &LoxValue::String(ref r)) => {
                    Ok(LoxValue::String(Cow::from(format!("{l}{r}"))))
                }
                _ => Err(LoxError::new_runtime_error(
                    "Operands must be two numbers or two strings.",
                )),
            },
            &TokenKind::Slash => Ok(LoxValue::Number(
                left_value.expect_number()? / right_value.expect_number()?,
            )),
            &TokenKind::Star => Ok(LoxValue::Number(
                left_value.expect_number()? * right_value.expect_number()?,
            )),
            &TokenKind::Greater => Ok(LoxValue::Boolean(
                left_value.expect_number()? > right_value.expect_number()?,
            )),
            &TokenKind::GreaterEqual => Ok(LoxValue::Boolean(
                left_value.expect_number()? >= right_value.expect_number()?,
            )),
            &TokenKind::Less => Ok(LoxValue::Boolean(
                left_value.expect_number()? < right_value.expect_number()?,
            )),
            &TokenKind::LessEqual => Ok(LoxValue::Boolean(
                left_value.expect_number()? <= right_value.expect_number()?,
            )),
            &TokenKind::EqualEqual => Ok(LoxValue::Boolean(left_value == right_value)),
            &TokenKind::BangEqual => Ok(LoxValue::Boolean(left_value != right_value)),
            _ => panic!("{} on numbers is not supported", op.kind),
        }
    }
}

impl<'a> LoxValue<'a> {
    fn expect_number(&self) -> Result<f64, LoxError> {
        match self {
            &LoxValue::Number(n) => Ok(n),
            _ => Err(LoxError::new_runtime_error(&format!(
                "Operand '{}' must be a number.",
                self,
            ))),
        }
    }

    fn is_truthy(&self) -> bool {
        match self {
            &LoxValue::Nil => false,
            &LoxValue::Boolean(tf) => tf,
            _ => true,
        }
    }
}
