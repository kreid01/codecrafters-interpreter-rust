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
        // Expression::Binary(left, operator, right) => binary(*left, operator, *right),
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

fn binary(left: Expression, operator: Operator, right: Expression) -> Result<String, String> {
    Ok("win".to_string())
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
