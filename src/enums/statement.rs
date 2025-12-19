use std::fmt::{self, Display};

use crate::enums::expression::Expression;

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
}

impl Display for Statement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Print(expression) => write!(fmt, "{}", expression),
            Statement::Expression(expression) => write!(fmt, "{}", expression),
        }
    }
}
