use std::collections::VecDeque;

use crate::expression::{Expression, Operator, Primary};
use crate::parser::parse;

pub fn execute(filename: &str) {
    let (expressions, _) = parse(filename);
    let expressions: VecDeque<Expression> = expressions.into();
    let mut stream = ExpressionStream { expressions };

    let mut output: Vec<String> = Vec::new();

    while let Some(expression) = stream.advance() {
        let result = match expression {
            Expression::Primary(literal) => primary(literal),
            // Expression::Binary(left, operator, right) => binary(*left, operator, *right),
            _ => panic!("Not implemented"),
        };

        output.push(result);
    }

    for s in output {
        println!("{}", s);
    }
}

fn primary(literal: Primary) -> String {
    match literal {
        Primary::Number(_, number, _) => number.to_string(),
        _ => literal.to_string(),
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
