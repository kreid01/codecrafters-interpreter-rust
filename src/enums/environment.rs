use crate::enums::error::Error;
use crate::enums::statement::Statement;
use crate::enums::token::Token;
use crate::evaluator::Value;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type Env = Rc<RefCell<Environment>>;

#[derive(Debug)]
pub struct Environment {
    pub symbols: HashMap<String, Symbol>,
    pub enclosing: Option<Env>,
}

#[derive(Debug, Clone)]
pub enum Symbol {
    Variable(Value),
    Function(Vec<Token>, Statement),
}

impl Environment {
    pub fn new() -> Env {
        Rc::new(RefCell::new(Environment {
            symbols: HashMap::new(),
            enclosing: None,
        }))
    }

    pub fn with_enclosing(enclosing: Env) -> Env {
        Rc::new(RefCell::new(Environment {
            symbols: HashMap::new(),
            enclosing: Some(enclosing),
        }))
    }

    pub fn define(&mut self, name: String, value: Symbol) {
        self.symbols.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Symbol> {
        if let Some(v) = self.symbols.get(name) {
            return Some(v.clone());
        }

        self.enclosing
            .as_ref()
            .and_then(|parent| parent.borrow().get(name))
    }

    pub fn assign(&mut self, name: &str, value: Symbol) -> Result<(), Error> {
        if self.symbols.contains_key(name) {
            self.symbols.insert(name.to_string(), value);
            Ok(())
        } else if let Some(parent) = self.enclosing.as_ref() {
            parent.borrow_mut().assign(name, value)
        } else {
            Err(Error::RuntimeError(
                1,
                format!("Undefined variable '{}'", name),
            ))
        }
    }
}
