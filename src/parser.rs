use std::collections::VecDeque;
use std::fmt::{self, Display};

use crate::expression::{self, Expression, Literal, Operator, Unary};
use crate::tokenizer::tokenize;
use crate::tokens::{AsString, Token, TokenStream};

fn precedence(op: &Operator) -> u8 {
    match op {
        Operator::Plus | Operator::Minus => 1,
        Operator::Star | Operator::Division => 2,
        _ => 0,
    }
}

pub fn parse(filename: &str) {
    let (tokens, errors) = tokenize(filename);
    let mut ast: Vec<Expression> = Vec::new();

    let tokens: VecDeque<Token> = tokens.into();
    let mut stream = TokenStream { tokens };

    while let Some(token) = stream.next() {
        if let Some(expr) = parse_token(token, &mut stream) {
            ast.push(expr);
        }
    }

    for syntax in &ast {
        println!("{}", syntax);
    }
}

fn parse_token(token: Token, tokens: &mut TokenStream) -> Option<Expression> {
    match token {
        Token::String(literal) => get_string_literal_expression(literal),
        Token::Number(_, number) => get_numeric_expression(number, tokens),
        Token::LeftParen => get_group_expression(tokens),
        Token::Bang => get_unary_expression(Unary::Bang, tokens),
        Token::Minus => get_unary_expression(Unary::Minus, tokens),
        Token::False => Some(Expression::Literal(Literal::False)),
        Token::True => Some(Expression::Literal(Literal::True)),
        _ => get_string_literal_expression(token.literal().to_string()),
    }
}

fn get_group_expression(tokens: &mut TokenStream) -> Option<Expression> {
    let mut expression: Option<Expression> = None;

    while let Some(next) = tokens.next() {
        match next {
            Token::RightParen => {
                if let Some(expr) = expression {
                    return Some(Expression::Grouping(Box::new(expr)));
                }
            }
            token => expression = parse_token(token, tokens),
        }
    }

    if let Some(expr) = expression {
        return Some(Expression::Grouping(Box::new(expr)));
    }

    None
}

fn get_numeric_expression(literal: String, tokens: &mut TokenStream) -> Option<Expression> {
    let mut expressions = VecDeque::from([Expression::Literal(Literal::Number(literal))]);
    let mut operators: VecDeque<Operator> = VecDeque::new();

    while let Some(next) = tokens.next() {
        match next {
            Token::Number(_, literal) => {
                expressions.push_back(Expression::Literal(Literal::Number(literal.to_string())));
            }
            Token::Minus => match get_unary_expression(Unary::Minus, tokens) {
                Some(expr) => {
                    expressions.push_back(expr);
                }
                None => {
                    return None;
                }
            },
            Token::Plus => operators.push_back(Operator::Plus),
            Token::Star => operators.push_back(Operator::Star),
            Token::Division => operators.push_back(Operator::Division),
            Token::LeftParen => match get_group_expression(tokens) {
                Some(expr) => {
                    expressions.push_back(expr);
                }
                None => {
                    return None;
                }
            },
            _ => {
                return expressions.pop_front();
            }
        }

        if expressions.len() > 1 && !operators.is_empty() {
            let expr1 = expressions.pop_front().unwrap();
            let expr2 = expressions.pop_front().unwrap();
            let op = operators.pop_front().unwrap();
            expressions.push_back(Expression::Binary(Box::new(expr1), op, Box::new(expr2)))
        }
    }

    expressions.pop_front()
}

fn get_string_literal_expression(literal: String) -> Option<Expression> {
    Some(Expression::Literal(Literal::String(literal)))
}

fn get_unary_expression(unary: Unary, tokens: &mut TokenStream) -> Option<Expression> {
    let expression = match tokens.next() {
        Some(Token::True) => Expression::Unary(unary, Box::new(Expression::Literal(Literal::True))),
        Some(Token::False) => {
            Expression::Unary(unary, Box::new(Expression::Literal(Literal::False)))
        }
        Some(Token::Number(_, literal)) => Expression::Unary(
            unary,
            Box::new(Expression::Literal(Literal::Number(literal))),
        ),
        Some(token) => {
            if let Some(expr) = parse_token(token, tokens) {
                return Some(Expression::Unary(unary, Box::new(expr)));
            }
            return None;
        }
        _ => return None,
    };
    Some(expression)
}
