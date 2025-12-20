use std::collections::HashMap;
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
    let mut symbols: HashMap<String, Value> = HashMap::new();

    for statement in statements {
        match statement {
            Statement::Print(expression) => match evaluate(&expression, &mut symbols) {
                Ok(val) => println!("{}", val),
                Err(err) => {
                    errors.push(err);
                }
            },
            Statement::Expression(expression) => match evaluate(&expression, &mut symbols) {
                Ok(_val) => {}
                Err(err) => {
                    errors.push(err);
                }
            },
            Statement::Declaration(name, expression) => match evaluate(&expression, &mut symbols) {
                Ok(val) => {
                    symbols.insert(name, val);
                }
                Err(err) => {
                    errors.push(err);
                }
            },
        }
    }
}
