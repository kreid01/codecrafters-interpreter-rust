use core::panic;
use std::collections::VecDeque;

use crate::expression::{Expression, Operator, Primary, Unary};
use crate::parser::parse;

pub fn execute(filename: &str) {
    let (expressions, _) = parse(filename);
    let expressions: VecDeque<Expression> = expressions.into();
    let mut stream = ExpressionStream { expressions };

    let mut output: Vec<String> = Vec::new();

    while !stream.is_at_end() {
        let result = evaluate_expression(stream.advance().expect("Tokens to exist"), &mut stream);
        output.push(result);
    }

    for s in output {
        println!("{}", s);
    }
}

fn evaluate_expression(expression: Expression, stream: &mut ExpressionStream) -> String {
    match expression {
        Expression::Primary(literal) => primary(literal, stream),
        Expression::Unary(operator, expression) => unary(operator, *expression, stream),
        Expression::Binary(left, operator, right) => binary(*left, operator, *right, stream),
        _ => panic!("Not implemented"),
    }
}

fn primary(literal: Primary, stream: &mut ExpressionStream) -> String {
    match literal {
        Primary::Number(_, number, _) => number.to_string(),
        Primary::String(string, _) => string.to_string(),
        Primary::Grouping(expression) => evaluate_expression(*expression, stream),
        _ => literal.to_string(),
    }
}

fn unary(unary: Unary, expression: Expression, stream: &mut ExpressionStream) -> String {
    let expression = evaluate_expression(expression, stream);

    match unary {
        Unary::Minus => {
            format!("{}{}", unary, expression)
        }
        Unary::Bang => check_bang(expression, stream),
    }
}

fn check_bang(expression: String, stream: &mut ExpressionStream) -> String {
    let expression = match stream.peek() {
        Some(Expression::Unary(Unary::Bang, _)) => {
            stream.advance();
            return check_bang(expression, stream);
        }
        _ => expression,
    };

    match expression.as_str() {
        "true" => "false".to_string(),
        "nil" => "true".to_string(),
        "false" => "true".to_string(),
        _ => "false".to_string(),
    }
}

fn binary(
    left: Expression,
    operator: Operator,
    right: Expression,
    stream: &mut ExpressionStream,
) -> String {
    let left = evaluate_expression(left, stream);
    let right = evaluate_expression(right, stream);

    let left = check_double_negative(left);
    let right = check_double_negative(right);

    match get_numeric_expressions(&left, &right) {
        Some((left, right)) => arithmetic(left, operator, right),
        None => concat(left, operator, right),
    }
}

fn concat(left: String, operator: Operator, right: String) -> String {
    match operator {
        Operator::Plus => format!("{}{}", left, right),
        _ => panic!("Unable to execute operator {} on strings", operator),
    }
}

fn get_numeric_expressions(left: &str, right: &str) -> Option<(f64, f64)> {
    let left = match left.parse::<f64>() {
        Ok(left) => left,
        Err(_) => {
            return None;
        }
    };

    let right = match right.parse::<f64>() {
        Ok(right) => right,
        Err(_) => {
            return None;
        }
    };

    Some((left, right))
}

fn check_double_negative(number: String) -> String {
    match number.starts_with("--") {
        true => number.replace("--", ""),
        false => number,
    }
}

fn arithmetic(left: f64, operator: Operator, right: f64) -> String {
    match operator {
        Operator::Plus => (left + right).to_string(),
        Operator::Minus => (left - right).to_string(),
        Operator::Star => (left * right).to_string(),
        Operator::Division => (left / right).to_string(),
        Operator::Less => (left < right).to_string(),
        Operator::LessEqual => (left <= right).to_string(),
        Operator::Greater => (left > right).to_string(),
        Operator::GreaterEqual => (left >= right).to_string(),
        _ => panic!("Not implemented operator"),
    }
}

impl Expression {
    pub fn variant_matches(&self, other: &Expression) -> bool {
        self == other
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

    pub fn peek_is(&self, expected_expression_type: &Expression) -> bool {
        match self.peek() {
            Some(expression) => expression.variant_matches(expected_expression_type),
            None => false,
        }
    }

    pub fn match_advance(&mut self, expected_expression_type: &Expression) -> bool {
        if self.peek_is(expected_expression_type) {
            self.advance();
            return true;
        }
        false
    }
}
