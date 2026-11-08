use crate::{
    Env, Expr, LoxError, LoxValue, Stmt, Token, TokenKind, loxerror::SourceLocation,
    parser::ParseResult,
};
use std::borrow::Cow;

pub struct Interpreter<'i, P>
where
    P: Iterator<Item = ParseResult<'i>>,
{
    parser: P,
    env: Env<'i>,
}

impl<'i, P> Interpreter<'i, P>
where
    P: Iterator<Item = ParseResult<'i>>,
{
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            env: Env::default(),
        }
    }

    pub fn run(&mut self) -> Result<(), LoxError> {
        while let Some(parse_result) = self.parser.next() {
            match parse_result? {
                Some(stmt) => self.exec(stmt)?,
                None => return Ok(()),
            }
        }

        Ok(())
    }

    fn exec(&mut self, stmt: Stmt<'i>) -> Result<(), LoxError> {
        match stmt {
            Stmt::Print(expr) => Ok(Self::print_lox_value(&self.eval_expr(&expr)?)),
            Stmt::Expr(expr) => self.eval_expr(&expr).map(|_| ()), // side effect
            Stmt::Var { name, initializer } => {
                let value: LoxValue<'_> = match initializer {
                    Some(init) => self.eval_expr(&init)?,
                    None => LoxValue::Nil,
                };
                Ok(self.env.define(name.lexeme, value))
            }
            Stmt::Block(stmts) => self.exec_block(stmts, Env::with_enclosing(self.env.clone())),
        }
    }

    fn exec_block(&mut self, stmts: Vec<Stmt<'i>>, mut env: Env<'i>) -> Result<(), LoxError> {
        std::mem::swap(&mut self.env, &mut env);

        for stmt in stmts {
            self.exec(stmt)?;
        }

        // swap back
        std::mem::swap(&mut env, &mut self.env);

        Ok(())
    }

    fn print_lox_value(value: &LoxValue<'_>) {
        match value {
            LoxValue::Nil => println!("nil"),
            LoxValue::Number(n) => {
                let s = format!("{n}");
                if s.ends_with(".0") {
                    println!("{}", &s[..s.len() - 2]);
                } else {
                    println!("{}", s);
                }
            }
            _ => println!("{value}"),
        };
    }

    fn eval_expr(&mut self, expr: &Expr<'i>) -> Result<LoxValue<'i>, LoxError> {
        match expr {
            &Expr::Literal(ref value) => Ok(value.clone()), // clone is cheap
            &Expr::Group(ref expr) => self.eval_expr(expr),
            &Expr::Unary(ref op, ref expr) => self.eval_unary_expr(op, expr),
            &Expr::Binary(ref left, ref op, ref right) => self.eval_binary_expr(left, op, right),
            &Expr::Var(ref name) => self.env.get_value(name).map(|v| v.clone()),
            &Expr::Assign(ref name, ref value_expr) => {
                let value = self.eval_expr(value_expr)?;
                _ = self.env.assign(name, value.clone())?;
                Ok(value)
            }
        }
    }

    fn eval_unary_expr(
        &mut self,
        op: &Token<'_>,
        expr: &Expr<'i>,
    ) -> Result<LoxValue<'i>, LoxError> {
        let value = self.eval_expr(expr)?;

        match &op.kind {
            &TokenKind::Minus => Ok(LoxValue::Number(-value.expect_number(op.line).map_err(
                |_| {
                    // we have to re-map the error message
                    LoxError::new_runtime_error(
                        "Operand must be a number.",
                        SourceLocation::Line(op.line),
                    )
                },
            )?)),
            &TokenKind::Bang => Ok(LoxValue::Boolean(!value.is_truthy())),
            _ => Err(LoxError::new_runtime_error(
                "invalid unary expression",
                SourceLocation::Token(&op),
            )),
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
                left_value.expect_number(op.line)? - right_value.expect_number(op.line)?,
            )),
            &TokenKind::Plus => match (&left_value, &right_value) {
                (&LoxValue::Number(ref l), &LoxValue::Number(ref r)) => Ok(LoxValue::Number(l + r)),
                (&LoxValue::String(ref l), &LoxValue::String(ref r)) => {
                    Ok(LoxValue::String(Cow::from(format!("{l}{r}"))))
                }
                _ => Err(LoxError::new_runtime_error(
                    "Operands must be two numbers or two strings.",
                    SourceLocation::Line(op.line),
                )),
            },
            &TokenKind::Slash => Ok(LoxValue::Number(
                left_value.expect_number(op.line)? / right_value.expect_number(op.line)?,
            )),
            &TokenKind::Star => Ok(LoxValue::Number(
                left_value.expect_number(op.line)? * right_value.expect_number(op.line)?,
            )),
            &TokenKind::Greater => Ok(LoxValue::Boolean(
                left_value.expect_number(op.line)? > right_value.expect_number(op.line)?,
            )),
            &TokenKind::GreaterEqual => Ok(LoxValue::Boolean(
                left_value.expect_number(op.line)? >= right_value.expect_number(op.line)?,
            )),
            &TokenKind::Less => Ok(LoxValue::Boolean(
                left_value.expect_number(op.line)? < right_value.expect_number(op.line)?,
            )),
            &TokenKind::LessEqual => Ok(LoxValue::Boolean(
                left_value.expect_number(op.line)? <= right_value.expect_number(op.line)?,
            )),
            &TokenKind::EqualEqual => Ok(LoxValue::Boolean(left_value == right_value)),
            &TokenKind::BangEqual => Ok(LoxValue::Boolean(left_value != right_value)),
            _ => panic!("{} on numbers is not supported", op.kind),
        }
    }
}

impl<'a> LoxValue<'a> {
    fn expect_number(&self, line: usize) -> Result<f64, LoxError> {
        match self {
            &LoxValue::Number(n) => Ok(n),
            _ => Err(LoxError::new_runtime_error(
                "Operands must be numbers.",
                SourceLocation::Line(line),
            )),
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
