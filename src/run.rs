use std::process;

use crate::enums::environment::Environment;
use crate::enums::error::Error;
use crate::enums::statement::Statement;
use crate::evaluator::{Value, evaluate};
use crate::parser::parse_statements;

pub fn run(filename: &str) {
    let (statements, errors) = parse_statements(filename);

    if !errors.is_empty() {
        process::exit(65)
    }

    let mut runtime_errors = Vec::new();
    let mut environment = Environment::new();

    evaluate_statements(statements, &mut runtime_errors, &mut environment);
}

fn evaluate_statements(
    statements: Vec<Statement>,
    errors: &mut Vec<Error>,
    environment: &mut Environment,
) {
    for statement in statements {
        evaluate_statement(statement, errors, environment);
    }
}

fn evaluate_statement(
    statement: Statement,
    errors: &mut Vec<Error>,
    environment: &mut Environment,
) {
    match statement {
        Statement::Print(expression) => match evaluate(&expression, environment) {
            Ok(val) => println!("{}", val),
            Err(err) => errors.push(err),
        },

        Statement::Expression(expression) => {
            if let Err(err) = evaluate(&expression, environment) {
                errors.push(err);
            }
        }

        Statement::Declaration(name, expression) => match evaluate(&expression, environment) {
            Ok(val) => environment.define(name, val),
            Err(err) => errors.push(err),
        },

        Statement::Block(statements) => {
            let mut block_env =
                Environment::with_enclosing(std::mem::replace(environment, Environment::new()));

            evaluate_statements(statements, errors, &mut block_env);

            *environment = *block_env.enclosing.unwrap();
        }

        Statement::IfElse(condition, if_stmt, else_stmt) => {
            let condition = match evaluate(&condition, environment) {
                Ok(val) => val,
                Err(err) => return errors.push(err),
            };

            let condition = match condition {
                Value::Boolean(bool) => bool,
                _ => {
                    return errors.push(Error::RuntimeError(
                        1,
                        "Only booleans can be conditional expression".to_string(),
                    ));
                }
            };

            if condition {
                evaluate_statement(*if_stmt, errors, environment);
            } else if let Some(else_stmt) = else_stmt {
                evaluate_statement(*else_stmt, errors, environment);
            }
        }
    }
}
