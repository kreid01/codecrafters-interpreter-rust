use std::process;

use crate::enums::expression::Expression;
use crate::enums::statement::Statement;
use crate::evaluator::evaluate;
use crate::parser::parse_statements;

pub fn run(filename: &str) {
    let (statements, errors) = parse_statements(filename);

    if !errors.is_empty() {
        process::exit(65)
    }

    for statement in statements {
        match statement {
            Statement::Print(expression) => {
                get_expression(expression);
            }
            Statement::Expression(expression) => {
                get_expression(expression);
            }
        }
    }
}

fn get_expression(expr: Expression) {
    match evaluate(&expr) {
        Ok(val) => println!("{}", val),
        Err(err) => println!("{}", err),
    }
}
