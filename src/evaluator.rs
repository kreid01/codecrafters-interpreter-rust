use crate::enums::error::Error;
use crate::enums::expression::{Expression, ExpressionStream, Operator, Primary, Unary};
use core::panic;

use std::fmt::{self, Display};

#[derive(PartialEq, Debug)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
}

impl Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Value::String(string) => write!(fmt, "{}", string),
            Value::Number(number) => write!(fmt, "{}", number),
            Value::Boolean(bool) => write!(fmt, "{}", bool),
        }
    }
}

pub fn evaluate(expressions: Expression) -> Result<Value, Error> {
    println!("{:?}", expressions);
    let mut stream = ExpressionStream { expressions };

    match evaluate_expression(stream.advance().expect("Tokens to exist"), &mut stream) {
        Ok(result) => Ok(result),
        Err(error) => {
            Err(error)
            // process::exit(70);
        }
    }
}

fn evaluate_expression(
    expression: Expression,
    stream: &mut ExpressionStream,
) -> Result<Value, Error> {
    match expression {
        Expression::Primary(literal) => primary(literal, stream),
        Expression::Unary(operator, expression) => unary(operator, *expression, stream),
        Expression::Binary(left, operator, right) => binary(*left, operator, *right, stream),
    }
}

fn primary(primary: Primary, stream: &mut ExpressionStream) -> Result<Value, Error> {
    match primary {
        Primary::Number(number) => Ok(Value::Number(number)),
        Primary::String(string) => Ok(Value::String(string)),
        Primary::True => Ok(Value::Boolean(true)),
        Primary::False => Ok(Value::Boolean(false)),
        Primary::Grouping(expression) => evaluate_expression(*expression, stream),
        _ => Ok(Value::String(primary.to_string())),
    }
}

fn unary(
    unary: Unary,
    expression: Expression,
    stream: &mut ExpressionStream,
) -> Result<Value, Error> {
    let expression = evaluate_expression(expression, stream)?;

    match unary {
        Unary::Minus => minus(expression),
        Unary::Bang => check_bang(expression, stream),
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

fn check_bang(expression: Value, stream: &mut ExpressionStream) -> Result<Value, Error> {
    let expression = match stream.peek() {
        Some(Expression::Unary(Unary::Bang, _)) => {
            stream.advance();
            return check_bang(expression, stream);
        }
        _ => expression,
    };

    let statement = match expression.to_string().as_str() {
        "true" => Value::Boolean(false),
        "nil" => Value::Boolean(true),
        "false" => Value::Boolean(true),
        _ => Value::Boolean(false),
    };

    Ok(statement)
}

fn binary(
    left: Expression,
    operator: Operator,
    right: Expression,
    stream: &mut ExpressionStream,
) -> Result<Value, Error> {
    let left = evaluate_expression(left, stream)?;
    let right = evaluate_expression(right, stream)?;

    let left = check_double_negative(left);
    let right = check_double_negative(right);

    match get_numeric_expressions(&left, &right) {
        Some((left, right)) => arithmetic(left, operator, right),
        None => operations(left, operator, right),
    }
}

fn operations(left: Value, operator: Operator, right: Value) -> Result<Value, Error> {
    match operator {
        Operator::Plus => plus(&left, &right),
        Operator::BangEqual => Ok(Value::Boolean(left.to_string() != right.to_string())),
        Operator::EqualEqual => Ok(Value::Boolean(equal(&left, &right))),
        _ => {
            let error = format!(
                "Unable to execute operator {} on strings ors booleans",
                operator
            );
            Err(Error::RuntimeError(1, error))
        }
    }
}

fn plus(left: &Value, right: &Value) -> Result<Value, Error> {
    match (left, right) {
        (Value::String(left), Value::String(right)) => {
            Ok(Value::String(format!("{}{}", left, right)))
        }
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

fn get_numeric_expressions(left: &Value, right: &Value) -> Option<(f64, f64)> {
    let left = match left {
        Value::Number(number) => number,
        _ => return None,
    };

    let right = match right {
        Value::Number(number) => number,
        _ => return None,
    };

    Some((*left, *right))
}

fn check_double_negative(statement: Value) -> Value {
    match statement {
        Value::String(ref string) => match string.starts_with("--") {
            true => Value::String(string.replace("--", "")),
            false => statement,
        },
        _ => statement,
    }
}

fn arithmetic(left: f64, operator: Operator, right: f64) -> Result<Value, Error> {
    match operator {
        Operator::Plus => Ok(Value::Number(left + right)),
        Operator::Minus => Ok(Value::Number(left - right)),
        Operator::Star => Ok(Value::Number(left * right)),
        Operator::Division => Ok(Value::Number(left / right)),
        Operator::Less => Ok(Value::Boolean(left < right)),
        Operator::LessEqual => Ok(Value::Boolean(left <= right)),
        Operator::Greater => Ok(Value::Boolean(left > right)),
        Operator::GreaterEqual => Ok(Value::Boolean(left >= right)),
        Operator::EqualEqual => Ok(Value::Boolean(left == right)),
        Operator::BangEqual => Ok(Value::Boolean(left != right)),
        _ => panic!("Not implemented operator"),
    }
}
