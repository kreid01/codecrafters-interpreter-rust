use crate::enums::error::Error;
use crate::enums::expression::{Expression, Operator, Primary, Unary};
use crate::enums::statement::Statement;
use crate::enums::token::{Token, TokenStream};
use crate::tokenizer::tokenize;
use std::collections::VecDeque;

pub fn parse(filename: &str) -> (Vec<Expression>, Vec<Error>) {
    let (tokens, _) = tokenize(filename);

    let mut expressions: Vec<Expression> = Vec::new();
    let tokens: VecDeque<Token> = tokens.into();
    let mut stream = TokenStream { tokens };
    let mut errors: Vec<Error> = Vec::new();

    while !stream.is_at_end() {
        match expression(&mut stream) {
            Ok(expression) => expressions.push(expression),
            Err(err) => {
                errors.push(err);
                break;
            }
        }
    }

    (expressions, errors)
}

pub fn parse_statements(filename: &str) -> (Vec<Statement>, Vec<Error>) {
    let (tokens, _) = tokenize(filename);

    let mut statements: Vec<Statement> = Vec::new();
    let tokens: VecDeque<Token> = tokens.into();
    let mut stream = TokenStream { tokens };
    let mut errors: Vec<Error> = Vec::new();

    while !stream.is_at_end() {
        match statement(&mut stream) {
            Ok(statement) => statements.push(statement),
            Err(err) => {
                errors.push(err);
                break;
            }
        }
    }

    (statements, errors)
}

fn statement(tokens: &mut TokenStream) -> Result<Statement, Error> {
    if tokens.match_advance(&Token::Print) {
        let expression = expression_statement(tokens)?;
        return Ok(Statement::Print(expression));
    }

    if tokens.match_advance(&Token::Var) {
        return declaration(tokens);
    }

    match expression_statement(tokens) {
        Ok(expression) => Ok(Statement::Expression(expression)),
        Err(err) => Err(err),
    }
}

fn declaration(tokens: &mut TokenStream) -> Result<Statement, Error> {
    let identifier = get_identifier(tokens)?;
    let next = peek(tokens);

    match next {
        Token::Equal => {
            tokens.advance();
            let expression = expression_statement(tokens)?;
            Ok(Statement::Declaration(identifier, expression))
        }
        Token::SemiColon => {
            tokens.advance();
            Ok(Statement::Declaration(
                identifier,
                Expression::Primary(Primary::Nil),
            ))
        }
        _ => Err(Error::RuntimeError(
            1,
            "Expected declaration or semi colon after identifier".to_string(),
        )),
    }
}

fn get_identifier(tokens: &mut TokenStream) -> Result<String, Error> {
    let next = peek(tokens);

    match next {
        Token::Identifier(identifier) => {
            tokens.advance();
            Ok(identifier)
        }
        _ => Err(Error::RuntimeError(
            1,
            "Expected variable name after var".to_string(),
        )),
    }
}

fn peek(tokens: &mut TokenStream) -> Token {
    match tokens.peek().cloned() {
        Some(next) => next,
        None => panic!("End of token stream"),
    }
}

fn expression_statement(tokens: &mut TokenStream) -> Result<Expression, Error> {
    let expression = expression(tokens)?;
    if tokens.match_advance(&Token::SemiColon) {
        Ok(expression)
    } else {
        let error = format!("Expect ';' after expression {}.", expression);
        Err(Error::ParseError(1, error))
    }
}

fn expression(tokens: &mut TokenStream) -> Result<Expression, Error> {
    assignment(tokens)
}

fn assignment(tokens: &mut TokenStream) -> Result<Expression, Error> {
    let left = equality(tokens)?;

    if tokens.peek_is(&Token::Equal) {
        tokens.advance();
        let right = assignment(tokens)?;

        match left {
            Expression::Primary(Primary::Identifier(name)) => {
                return Ok(Expression::Assignment(
                    Primary::Identifier(name),
                    Box::new(right),
                ));
            }
            _ => return Err(Error::ParseError(1, "Invalid assignment".to_string())),
        }
    }

    Ok(left)
}

fn equality(tokens: &mut TokenStream) -> Result<Expression, Error> {
    let mut expr = comparison(tokens)?;

    while tokens.peek_is(&Token::EqualEqual) || tokens.peek_is(&Token::BangEqual) {
        let operator_token = tokens.advance().unwrap();
        let op = to_equality(operator_token);
        let right = comparison(tokens)?;

        expr = Expression::Binary(Box::new(expr), op, Box::new(right));
    }

    Ok(expr)
}

fn comparison(tokens: &mut TokenStream) -> Result<Expression, Error> {
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

fn addition(tokens: &mut TokenStream) -> Result<Expression, Error> {
    let mut expr = multiplication(tokens)?;

    while tokens.peek_is(&Token::Plus) || tokens.peek_is(&Token::Minus) {
        let operator_token = tokens.advance().unwrap();
        let op = to_operator(operator_token);
        let right = multiplication(tokens)?;

        expr = Expression::Binary(Box::new(expr), op, Box::new(right));
    }

    Ok(expr)
}

fn multiplication(tokens: &mut TokenStream) -> Result<Expression, Error> {
    let mut expr = unary(tokens)?;

    while tokens.peek_is(&Token::Star) || tokens.peek_is(&Token::Division) {
        let operator_token = tokens.advance().unwrap();
        let op = to_operator(operator_token);
        let right = unary(tokens)?;

        expr = Expression::Binary(Box::new(expr), op, Box::new(right));
    }

    Ok(expr)
}

fn unary(tokens: &mut TokenStream) -> Result<Expression, Error> {
    if tokens.peek_is(&Token::Bang) || tokens.peek_is(&Token::Minus) {
        let operator_token = tokens.advance().unwrap();
        let unary_op = to_unary(operator_token);

        let right_operand = unary(tokens)?;

        Ok(Expression::Unary(unary_op, Box::new(right_operand)))
    } else {
        primary(tokens)
    }
}

fn primary(tokens: &mut TokenStream) -> Result<Expression, Error> {
    let token = match tokens.advance() {
        Some(token) => token,
        None => {
            panic!("End of token stream");
        }
    };

    match token {
        Token::False => Ok(Expression::Primary(Primary::False)),
        Token::True => Ok(Expression::Primary(Primary::True)),
        Token::Nil => Ok(Expression::Primary(Primary::Nil)),
        Token::Number(_, ref number) => Ok(Expression::Primary(Primary::Number(*number))),
        Token::String(ref literal) => Ok(Expression::Primary(Primary::String(literal.to_string()))),
        Token::Identifier(identifier) => Ok(Expression::Primary(Primary::Identifier(identifier))),

        Token::LeftParen => {
            let expr_inside = expression(tokens)?;
            if tokens.match_advance(&Token::RightParen) {
                Ok(Expression::Primary(Primary::Grouping(Box::new(
                    expr_inside,
                ))))
            } else {
                let error = Error::ParseError(1, "Expected ')' after expression.".to_string());
                Err(error)
            }
        }

        _ => {
            let error = Error::ParseError(1, format!("Unknown token {}", token));
            Err(error)
        }
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
