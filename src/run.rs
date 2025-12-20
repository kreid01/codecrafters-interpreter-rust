use std::process;

use crate::enums::error::Error;
use crate::enums::statement::Statement;
use crate::evaluator::{Value, evaluate};
use crate::parser::parse_statements;

pub fn run(filename: &str) {
    let (statements, errors) = parse_statements(filename);

    if !errors.is_empty() {
        process::exit(65)
    }

    let mut errors: Vec<Error> = Vec::new();
    let mut values: Vec<Value> = Vec::new();

    for statement in statements {
        match statement {
            Statement::Print(expression) => match evaluate(&expression) {
                Ok(val) => println!("{}", val),
                Err(err) => {
                    errors.push(err);
                }
            },
            Statement::Expression(expression) => match evaluate(&expression) {
                Ok(val) => {
                    values.push(val);
                }
                Err(err) => {
                    errors.push(err);
                }
            },
        }
    }
}
