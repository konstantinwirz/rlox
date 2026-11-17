use std::collections::HashMap;

use crate::{LoxError, LoxValue, Token, loxerror::SourceLocation};

#[derive(Debug, Default, Clone)]
pub struct Env<'env> {
    values: HashMap<String, LoxValue<'env>>,
    enclosing: Option<Box<Env<'env>>>,
}

impl<'env> Env<'env> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_enclosing(enclosing: Env<'env>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn define(&mut self, name: &str, value: LoxValue<'env>) {
        self.values.insert(name.to_string(), value);
    }

    pub fn assign(&mut self, name: &Token<'_>, value: LoxValue<'env>) -> Result<(), LoxError> {
        if self.values.contains_key(name.lexeme) {
            self.values.insert(name.lexeme.to_string(), value);
            return Ok(());
        }

        match &mut self.enclosing {
            Some(enclosing) => enclosing.assign(name, value),
            None => Err(LoxError::new_runtime_error(
                &format!("Undefined variable '{}'.", name.lexeme),
                SourceLocation::Line(name.line),
            )),
        }
    }

    pub fn get_value(&self, name: &Token<'_>) -> Result<&LoxValue<'env>, LoxError> {
        match self.values.get(name.lexeme) {
            Some(value) => Ok(value),
            None => match &self.enclosing {
                Some(enclosing) => enclosing.get_value(name),
                None => Err(LoxError::new_runtime_error(
                    &format!("Undefined variable '{}'.", name.lexeme),
                    SourceLocation::Line(name.line),
                )),
            },
        }
    }
}
