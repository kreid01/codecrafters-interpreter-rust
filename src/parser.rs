use crate::expression::{Expression, Operator, Primary, Unary};
use crate::tokenizer::tokenize;
use crate::tokens::{Token, TokenStream};
use std::collections::VecDeque;
use std::process;

pub fn parse(filename: &str) {
    let (tokens, errors) = tokenize(filename);
    let mut ast: Vec<Expression> = Vec::new();
    let tokens: VecDeque<Token> = tokens.into();
    let mut stream = TokenStream { tokens };

    if !errors.is_empty() {
        for error in errors {
            eprintln!("{}", error);
        }

        process::exit(65)
    }

    let mut errors: Vec<String> = Vec::new();

    while !stream.is_at_end() {
        match expression(&mut stream) {
            Ok(leaf) => ast.push(leaf),
            Err(err) => {
                errors.push(err);
                break;
            }
        }
    }

    if !errors.is_empty() {
        for error in errors {
            eprintln!("{}", error);
        }

        process::exit(65)
    }

    for syntax in &ast {
        println!("{}", syntax);
    }
}

fn expression(tokens: &mut TokenStream) -> Result<Expression, String> {
    equality(tokens)
}

fn equality(tokens: &mut TokenStream) -> Result<Expression, String> {
    let mut expr = comparison(tokens)?;

    while tokens.peek_is(&Token::EqualEqual) || tokens.peek_is(&Token::BangEqual) {
        let operator_token = tokens.advance().unwrap();
        let op = to_equality(operator_token);
        let right = comparison(tokens)?;

        expr = Expression::Binary(Box::new(expr), op, Box::new(right));
    }

    Ok(expr)
}

fn comparison(tokens: &mut TokenStream) -> Result<Expression, String> {
    let mut expr = addition(tokens)?;

    while tokens.peek_is(&Token::Less)
        || tokens.peek_is(&Token::LessEqual)
        || tokens.peek_is(&Token::Greater)
        || tokens.peek_is(&Token::GreaterEqual)
    {
        let operator_token = tokens.advance().unwrap();
        let op = to_comparison(operator_token);
        let right = addition(tokens)?;

        expr = Expression::Binary(Box::new(expr), op, Box::new(right));
    }

    Ok(expr)
}

fn addition(tokens: &mut TokenStream) -> Result<Expression, String> {
    let mut expr = multiplication(tokens)?;

    while tokens.peek_is(&Token::Plus) || tokens.peek_is(&Token::Minus) {
        let operator_token = tokens.advance().unwrap();
        let op = to_operator(operator_token);
        let right = multiplication(tokens)?;

        expr = Expression::Binary(Box::new(expr), op, Box::new(right));
    }

    Ok(expr)
}

fn multiplication(tokens: &mut TokenStream) -> Result<Expression, String> {
    let mut expr = unary(tokens)?;

    while tokens.peek_is(&Token::Star) || tokens.peek_is(&Token::Division) {
        let operator_token = tokens.advance().unwrap();
        let op = to_operator(operator_token);
        let right = unary(tokens)?;

        expr = Expression::Binary(Box::new(expr), op, Box::new(right));
    }

    Ok(expr)
}

fn unary(tokens: &mut TokenStream) -> Result<Expression, String> {
    if tokens.peek_is(&Token::Bang) || tokens.peek_is(&Token::Minus) {
        let operator_token = tokens.advance().unwrap();
        let unary_op = to_unary(operator_token);

        let right_operand = unary(tokens)?;

        Ok(Expression::Unary(unary_op, Box::new(right_operand)))
    } else {
        primary(tokens)
    }
}

fn primary(tokens: &mut TokenStream) -> Result<Expression, String> {
    let token = match tokens.advance() {
        Some(token) => token,
        None => {
            return Err("Error - end of token stream".to_string());
        }
    };

    match token {
        Token::False => Ok(Expression::Primary(Primary::False)),
        Token::True => Ok(Expression::Primary(Primary::True)),
        Token::Nil => Ok(Expression::Primary(Primary::Nil)),
        Token::Number(_, literal) => Ok(Expression::Primary(Primary::Number(literal))),
        Token::String(literal) => Ok(Expression::Primary(Primary::String(literal))),

        Token::LeftParen => {
            let expr_inside = expression(tokens)?;
            if tokens.match_advance(&Token::RightParen) {
                Ok(Expression::Primary(Primary::Grouping(Box::new(
                    expr_inside,
                ))))
            } else {
                let error = format!("[line 1] Error at '{}': Expect expression.", token);
                Err(error)
            }
        }

        _ => Err("Error -  unexpected token".to_string()),
    }
}

fn to_equality(token: Token) -> Operator {
    match token {
        Token::EqualEqual => Operator::EqualEqual,
        Token::BangEqual => Operator::BangEqual,
        _ => panic!("Expected binary operator, got {:?}", token),
    }
}

fn to_operator(token: Token) -> Operator {
    match token {
        Token::Star => Operator::Star,
        Token::Division => Operator::Division,
        Token::Minus => Operator::Minus,
        Token::Plus => Operator::Plus,
        _ => panic!("Expected binary operator, got {:?}", token),
    }
}

fn to_unary(token: Token) -> Unary {
    match token {
        Token::Bang => Unary::Bang,
        Token::Minus => Unary::Minus,
        _ => panic!("Expected unary operator, got {:?}", token),
    }
}

fn to_comparison(token: Token) -> Operator {
    match token {
        Token::Less => Operator::Less,
        Token::LessEqual => Operator::LessEqual,
        Token::Greater => Operator::Greater,
        Token::GreaterEqual => Operator::GreaterEqual,
        _ => panic!("Expected comparison operator, got {:?}", token),
    }
}
