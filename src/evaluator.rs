use core::panic;
use std::collections::VecDeque;
use std::fmt::{self, Display};

use crate::enums::expression::{Expression, Operator, Primary, Unary};
use crate::parser::parse;

#[derive(PartialEq)]
enum Statement {
    String(String),
    Number(f64),
    Boolean(bool),
}

impl Display for Statement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Statement::String(string) => write!(fmt, "{}", string),
            Statement::Number(number) => write!(fmt, "{}", number),
            Statement::Boolean(bool) => write!(fmt, "{}", bool),
        }
    }
}

fn equal(left: Statement, right: Statement) -> Statement {
    let equal = match (left, right) {
        (Statement::String(string1), Statement::String(string2)) => string1 == string2,
        (Statement::Number(number1), Statement::Number(number2)) => number1 == number2,
        (Statement::Boolean(bool1), Statement::Boolean(bool2)) => bool1 == bool2,
        _ => false,
    };

    Statement::Boolean(equal)
}

pub fn evaluate(filename: &str) {
    let (expressions, _) = parse(filename);
    let expressions: VecDeque<Expression> = expressions.into();
    let mut stream = ExpressionStream { expressions };

    let mut output: Vec<Statement> = Vec::new();

    while !stream.is_at_end() {
        let result = evaluate_expression(stream.advance().expect("Tokens to exist"), &mut stream);
        output.push(result);
    }

    for s in output {
        println!("{}", s);
    }
}

fn evaluate_expression(expression: Expression, stream: &mut ExpressionStream) -> Statement {
    match expression {
        Expression::Primary(literal) => primary(literal, stream),
        Expression::Unary(operator, expression) => unary(operator, *expression, stream),
        Expression::Binary(left, operator, right) => binary(*left, operator, *right, stream),
        _ => panic!("Not implemented"),
    }
}

fn primary(literal: Primary, stream: &mut ExpressionStream) -> Statement {
    match literal {
        Primary::Number(number, _) => Statement::Number(number),
        Primary::String(string, _) => Statement::String(string),
        Primary::Grouping(expression) => evaluate_expression(*expression, stream),
        _ => Statement::String(literal.to_string()),
    }
}

fn unary(unary: Unary, expression: Expression, stream: &mut ExpressionStream) -> Statement {
    let expression = evaluate_expression(expression, stream);

    match unary {
        Unary::Minus => minus(expression),
        Unary::Bang => check_bang(expression, stream),
    }
}

fn minus(statement: Statement) -> Statement {
    match statement {
        Statement::Number(number) => Statement::Number(-number),
        _ => Statement::String(format!("-{}", statement)),
    }
}

fn check_bang(expression: Statement, stream: &mut ExpressionStream) -> Statement {
    let expression = match stream.peek() {
        Some(Expression::Unary(Unary::Bang, _)) => {
            stream.advance();
            return check_bang(expression, stream);
        }
        _ => expression,
    };

    match expression.to_string().as_str() {
        "true" => Statement::Boolean(false),
        "nil" => Statement::Boolean(true),
        "false" => Statement::Boolean(true),
        _ => Statement::Boolean(false),
    }
}

fn binary(
    left: Expression,
    operator: Operator,
    right: Expression,
    stream: &mut ExpressionStream,
) -> Statement {
    let left = evaluate_expression(left, stream);
    let right = evaluate_expression(right, stream);

    let left = check_double_negative(left);
    let right = check_double_negative(right);

    match get_numeric_expressions(&left, &right) {
        Some((left, right)) => arithmetic(left, operator, right),
        None => string_operations(left, operator, right),
    }
}

fn string_operations(left: Statement, operator: Operator, right: Statement) -> Statement {
    match operator {
        Operator::Plus => Statement::String(format!("{}{}", left, right)),
        Operator::BangEqual => Statement::Boolean(left.to_string() != right.to_string()),
        Operator::EqualEqual => equal(left, right),
        _ => panic!("Unable to execute operator {} on strings", operator),
    }
}

fn get_numeric_expressions(left: &Statement, right: &Statement) -> Option<(f64, f64)> {
    let left = match left {
        Statement::Number(number) => number,
        _ => return None,
    };

    let right = match right {
        Statement::Number(number) => number,
        _ => return None,
    };

    Some((*left, *right))
}

fn check_double_negative(statement: Statement) -> Statement {
    match statement {
        Statement::String(ref string) => match string.starts_with("--") {
            true => Statement::String(string.replace("--", "")),
            false => statement,
        },
        _ => statement,
    }
}

fn arithmetic(left: f64, operator: Operator, right: f64) -> Statement {
    match operator {
        Operator::Plus => Statement::Number(left + right),
        Operator::Minus => Statement::Number(left - right),
        Operator::Star => Statement::Number(left * right),
        Operator::Division => Statement::Number(left / right),
        Operator::Less => Statement::Boolean(left < right),
        Operator::LessEqual => Statement::Boolean(left <= right),
        Operator::Greater => Statement::Boolean(left > right),
        Operator::GreaterEqual => Statement::Boolean(left >= right),
        Operator::EqualEqual => Statement::Boolean(left == right),
        Operator::BangEqual => Statement::Boolean(left != right),
        _ => panic!("Not implemented operator"),
    }
}

pub struct ExpressionStream {
    pub expressions: VecDeque<Expression>,
}

impl ExpressionStream {
    pub fn peek(&self) -> Option<&Expression> {
        self.expressions.front()
    }

    pub fn advance(&mut self) -> Option<Expression> {
        self.expressions.pop_front()
    }

    pub fn is_at_end(&self) -> bool {
        self.expressions.is_empty()
    }
}
