use std::process;

use crate::enums::environment::{Env, Environment, Symbol};
use crate::enums::error::Error;
use crate::enums::statement::Statement;
use crate::evaluator::{Value, evaluate, truthy};
use crate::parser::parse_statements;

#[derive(Debug)]
pub enum ControlFlow {
    Return(Value),
    Runtime(Error),
}

pub fn run(filename: &str) {
    let (statements, errors) = parse_statements(filename);

    if !errors.is_empty() {
        process::exit(65);
    }

    let mut environment = Environment::new();

    match evaluate_statements(statements, &mut environment) {
        Ok(()) => {}
        Err(ControlFlow::Runtime(err)) => {
            eprintln!("{}", err);
            process::exit(65);
        }
        Err(ControlFlow::Return(_)) => {
            eprintln!("Can't return from top-level code.");
            process::exit(70);
        }
    }
}

fn evaluate_statements(
    statements: Vec<Statement>,
    environment: &mut Env,
) -> Result<(), ControlFlow> {
    for statement in statements {
        evaluate_statement(statement, environment)?;
    }
    Ok(())
}

pub fn evaluate_statement(statement: Statement, environment: &mut Env) -> Result<(), ControlFlow> {
    match statement {
        Statement::Print(expr) => {
            let value = evaluate(&expr, environment).map_err(ControlFlow::Runtime)?;
            println!("{}", value);
            Ok(())
        }

        Statement::Expression(expr) => {
            evaluate(&expr, environment).map_err(ControlFlow::Runtime)?;
            Ok(())
        }

        Statement::Declaration(name, expr) => {
            let value = evaluate(&expr, environment).map_err(ControlFlow::Runtime)?;
            environment
                .borrow_mut()
                .define(name, Symbol::Variable(value));
            Ok(())
        }

        Statement::Block(statements) => {
            let mut block_env = Environment::with_enclosing(environment.clone());
            evaluate_statements(statements, &mut block_env)
        }

        Statement::IfElse(condition, then_stmt, else_stmt) => {
            let cond = evaluate(&condition, environment).map_err(ControlFlow::Runtime)?;
            if truthy(cond) {
                evaluate_statement(*then_stmt, environment)?;
            } else if let Some(else_stmt) = else_stmt {
                evaluate_statement(*else_stmt, environment)?;
            }
            Ok(())
        }

        Statement::While(condition, body) => {
            while truthy(evaluate(&condition, environment).map_err(ControlFlow::Runtime)?) {
                evaluate_statement(*body.clone(), environment)?
            }
            Ok(())
        }

        Statement::For(initializer, condition, increment, body) => {
            if let Some(init) = initializer {
                evaluate_statement(*init, environment)?;
            }

            loop {
                let cond = match condition.as_ref() {
                    Some(c) => truthy(evaluate(c, environment).map_err(ControlFlow::Runtime)?),
                    None => true,
                };

                if !cond {
                    break;
                }

                let mut body_env = Environment::with_enclosing(environment.clone());
                evaluate_statement(*body.clone(), &mut body_env)?;

                if let Some(inc) = increment.as_ref() {
                    evaluate(inc, environment).map_err(ControlFlow::Runtime)?;
                }
            }

            Ok(())
        }

        Statement::Fn(name, params, body) => {
            let function = Symbol::Function(params, *body);
            environment.borrow_mut().define(name, function);
            Ok(())
        }

        Statement::Return(expr) => {
            let value = evaluate(&expr, environment).map_err(ControlFlow::Runtime)?;
            Err(ControlFlow::Return(value))
        }
    }
}
