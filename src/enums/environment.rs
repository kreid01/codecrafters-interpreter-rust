use crate::enums::error::Error;
use crate::enums::statement::Statement;
use crate::enums::token::Token;
use crate::evaluator::Value;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Environment {
    pub symbols: HashMap<String, Symbol>,
    pub enclosing: Option<Box<Environment>>,
}

#[derive(Debug, Clone)]
pub enum Symbol {
    Variable(Value),
    Function(Vec<Token>, Statement),
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            symbols: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn with_enclosing(enclosing: Environment) -> Self {
        Environment {
            symbols: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn define(&mut self, name: String, value: Symbol) {
        self.symbols.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Symbol> {
        if let Some(v) = self.symbols.get(name) {
            return Some(v.clone());
        }

        if let Some(ref parent) = self.enclosing {
            return parent.get(name);
        }

        None
    }

    pub fn assign(&mut self, name: &str, value: Symbol) -> Result<(), Error> {
        if self.symbols.contains_key(name) {
            self.symbols.insert(name.to_string(), value);
            return Ok(());
        }

        if let Some(ref mut parent) = self.enclosing {
            return parent.assign(name, value);
        }

        Err(Error::RuntimeError(
            1,
            format!("Undefined variable '{}'", name),
        ))
    }
}
