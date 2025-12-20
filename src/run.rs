use std::process;

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
                if let Ok(eval) = evaluate(&expression) {
                    println!("{}", eval);
                }
            }
            Statement::Expression(expression) => {
                if let Ok(eval) = evaluate(&expression) {
                    println!("{}", eval);
                }
            }
        }
    }
}
