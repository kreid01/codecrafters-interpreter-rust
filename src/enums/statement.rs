use std::fmt::{self, Display};

use crate::enums::expression::Expression;
use crate::enums::token::Token;

#[derive(Debug, Clone)]
pub enum Statement {
    Block(Vec<Statement>),
    Declaration(String, Expression),
    Expression(Expression),
    IfElse(Expression, Box<Statement>, Option<Box<Statement>>),
    Print(Expression),
    While(Expression, Box<Statement>),
    For(
        Option<Box<Statement>>,
        Option<Expression>,
        Option<Expression>,
        Box<Statement>,
    ),
    Fn(String, Vec<Token>, Box<Statement>),
}

impl Display for Statement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Block(statements) => write!(fmt, "{:?}", statements),
            Statement::Print(expression) => write!(fmt, "{}", expression),
            Statement::Expression(expression) => write!(fmt, "{}", expression),
            Statement::Declaration(string, expression) => {
                write!(fmt, "{} - {}", string, expression)
            }
            Statement::IfElse(conditional, if_stmt, _) => {
                write!(fmt, "if ({}) {} else ", conditional, if_stmt,)
            }
            Statement::While(conditional, statement) => {
                write!(fmt, "while ({}) {} ", conditional, statement)
            }
            Statement::For(statement, check, increment, block) => {
                write!(
                    fmt,
                    "for ({}{}{}) {} ",
                    statement.clone().unwrap(),
                    check.clone().unwrap(),
                    increment.clone().unwrap(),
                    block
                )
            }
            Statement::Fn(name, params, body) => {
                let params = params
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                write!(fmt, "{}({}) {}", name, params, body)
            }
        }
    }
}
