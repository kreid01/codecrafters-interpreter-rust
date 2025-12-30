use crate::enums::environment::{Env, Environment, Symbol};
use crate::enums::error::Error;
use crate::enums::expression::{Expression, Operator, Primary, Unary};
use crate::enums::statement::Statement;
use crate::enums::token::Token;
use crate::run::{evaluate_statement, ControlFlow};
use std::collections::VecDeque;

use std::fmt::{self, Display};
use std::time::SystemTime;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Value::String(string) => write!(fmt, "{}", string),
            Value::Number(number) => write!(fmt, "{}", number),
            Value::Boolean(bool) => write!(fmt, "{}", bool),
            Value::Nil => write!(fmt, "nil"),
        }
    }
}

pub fn evaluate(expression: &Expression, symbols: &mut Env) -> Result<Value, Error> {
    match expression {
        Expression::Primary(literal) => primary(literal, symbols),
        Expression::Unary(operator, expression) => unary(operator, expression, symbols),
        Expression::Binary(left, operator, right) => binary(left, operator, right, symbols),
        Expression::Assignment(identifier, expression) => {
            assignment(identifier, expression, symbols)
        }
    }
}

fn assignment(
    identifier: &Primary,
    expression: &Expression,
    environment: &mut Env,
) -> Result<Value, Error> {
    let value = evaluate(expression, environment)?;

    let name = match identifier {
        Primary::Identifier(name) => name,
        _ => {
            return Err(Error::RuntimeError(
                1,
                "Invalid assignment target".to_string(),
            ));
        }
    };

    environment
        .borrow_mut()
        .assign(name, Symbol::Variable(value.clone()))?;

    Ok(value)
}

fn primary(primary: &Primary, symbols: &mut Env) -> Result<Value, Error> {
    match primary {
        Primary::Number(number) => Ok(Value::Number(number.to_owned())),
        Primary::String(string) => Ok(Value::String(string.to_string())),
        Primary::True => Ok(Value::Boolean(true)),
        Primary::False => Ok(Value::Boolean(false)),
        Primary::Nil => Ok(Value::Nil),
        Primary::Grouping(expression) => evaluate(expression, symbols),
        Primary::Identifier(identifier) => variable(identifier, symbols),
        Primary::Function(name, arguments) => function(name, arguments.to_vec(), symbols),
    }
}

fn function(name: &str, args: Vec<Expression>, symbols: &Env) -> Result<Value, Error> {
    let symbol = symbols.borrow().get(name);

    if let Some(symbol) = symbol {
        match symbol {
            Symbol::Variable(_) => {
                return Err(Error::RuntimeError(1, "Variable not callable".to_string()));
            }
            Symbol::Function(params, body) => {
                return call_function(params, args, body, symbols.to_owned());
            }
        }
    }

    match name {
        "clock" => clock(),
        _ => Err(Error::RuntimeError(1, "Unknown method".to_string())),
    }
}

fn call_function(
    params: Vec<Token>,
    args: Vec<Expression>,
    body: Statement,
    closure_env: Env,
) -> Result<Value, Error> {
    let mut function_env = Environment::with_enclosing(closure_env);

    let mut arg_queue: VecDeque<Expression> = args.into();
    for param in params {
        let arg_expr = arg_queue.pop_front().unwrap();
        let arg_value = evaluate(&arg_expr, &mut function_env)?;
        function_env
            .borrow_mut()
            .define(param.get_identifier(), Symbol::Variable(arg_value));
    }

    match evaluate_statement(body, &mut function_env) {
        Ok(()) => Ok(Value::Nil),
        Err(ControlFlow::Return(v)) => Ok(v),
        Err(ControlFlow::Runtime(e)) => Err(e),
    }
}

fn clock() -> Result<Value, Error> {
    let now = SystemTime::now();
    if let Ok(now) = now.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(Value::Number(now.as_secs_f64()))
    } else {
        Err(Error::RuntimeError(
            1,
            "Failed to get current time".to_string(),
        ))
    }
}

fn variable(string: &str, symbols: &Env) -> Result<Value, Error> {
    match symbols.borrow().get(string) {
        Some(value) => match value {
            Symbol::Variable(val) => Ok(val),
            Symbol::Function(_, _) => {
                let val = format!("<fn {}>", string);
                Ok(Value::String(val))
            }
        },
        None => Err(Error::RuntimeError(1, "Unknown identifier".to_string())),
    }
}

fn unary(unary: &Unary, expression: &Expression, symbols: &mut Env) -> Result<Value, Error> {
    let expression = evaluate(expression, symbols)?;

    match unary {
        Unary::Minus => minus(expression),
        Unary::Bang => check_bang(expression),
    }
}

fn minus(statement: Value) -> Result<Value, Error> {
    match statement {
        Value::Number(number) => Ok(Value::Number(-number)),
        _ => {
            let error = "Operand must be a number.".to_string();
            Err(Error::RuntimeError(1, error))
        }
    }
}

fn check_bang(expression: Value) -> Result<Value, Error> {
    let statement = match expression.to_string().as_str() {
        "true" => Value::Boolean(false),
        "nil" => Value::Boolean(true),
        "false" => Value::Boolean(true),
        _ => Value::Boolean(false),
    };

    Ok(statement)
}

fn binary(
    left: &Expression,
    operator: &Operator,
    right: &Expression,
    symbols: &mut Env,
) -> Result<Value, Error> {
    let left = evaluate(left, symbols)?;

    //ugly please fix
    if matches!(operator, Operator::Or) {
        if truthy(left.clone()) {
            return Ok(left);
        } else {
            let right = evaluate(right, symbols)?;
            if truthy(right.clone()) {
                return Ok(right);
            } else {
                return Ok(Value::Boolean(false));
            }
        }
    }

    if matches!(operator, Operator::And) {
        if !truthy(left.clone()) {
            return Ok(left);
        } else {
            let right = evaluate(right, symbols)?;
            return Ok(right);
        }
    }

    let right = evaluate(right, symbols)?;

    match operator {
        Operator::Plus => plus(&left, &right),
        Operator::BangEqual => Ok(Value::Boolean(left != right)),
        Operator::EqualEqual => Ok(Value::Boolean(equal(&left, &right))),
        _ => {
            if let (Value::Number(left), Value::Number(right)) = (left, right) {
                return arithmetic(left, operator, right);
            }
            let error = format!(
                "Unable to execute operator {} on strings ors booleans",
                operator
            );
            Err(Error::RuntimeError(1, error))
        }
    }
}

fn arithmetic(left: f64, operator: &Operator, right: f64) -> Result<Value, Error> {
    match operator {
        Operator::Minus => Ok(Value::Number(left - right)),
        Operator::Star => Ok(Value::Number(left * right)),
        Operator::Division => Ok(Value::Number(left / right)),
        Operator::Less => Ok(Value::Boolean(left < right)),
        Operator::LessEqual => Ok(Value::Boolean(left <= right)),
        Operator::Greater => Ok(Value::Boolean(left > right)),
        Operator::GreaterEqual => Ok(Value::Boolean(left >= right)),
        Operator::EqualEqual => Ok(Value::Boolean(left == right)),
        Operator::BangEqual => Ok(Value::Boolean(left != right)),
        _ => panic!("Operator not recognizer"),
    }
}

fn plus(left: &Value, right: &Value) -> Result<Value, Error> {
    match (left, right) {
        (Value::String(left), Value::String(right)) => {
            Ok(Value::String(format!("{}{}", left, right)))
        }
        (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left + right)),
        _ => Err(Error::RuntimeError(
            1,
            "Opperands must be 2 numbers or 2 strings".to_string(),
        )),
    }
}

fn equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::String(string1), Value::String(string2)) => string1 == string2,
        (Value::Number(number1), Value::Number(number2)) => number1 == number2,
        (Value::Boolean(bool1), Value::Boolean(bool2)) => bool1 == bool2,
        _ => false,
    }
}

pub fn truthy(value: Value) -> bool {
    match value {
        Value::String(_) => true,
        Value::Boolean(bool) => bool,
        Value::Number(number) => number != 0.0,
        Value::Nil => false,
    }
}
