use crate::enums::error::Error;
use crate::enums::expression::{Expression, Operator, Primary, Unary};
use crate::enums::statement::Statement;
use crate::enums::token::{Lexeme, Token, TokenStream};
use crate::tokenizer::tokenize;
use std::collections::VecDeque;

pub fn parse(filename: &str) -> (Vec<Expression>, Vec<Error>) {
    let (tokens, _) = tokenize(filename);

    let mut expressions: Vec<Expression> = Vec::new();
    let tokens: VecDeque<Lexeme> = tokens.into();
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
    let tokens: VecDeque<Lexeme> = tokens.into();
    let mut stream = TokenStream { tokens };
    let mut errors: Vec<Error> = Vec::new();

    while !stream.is_at_end() {
        match block(&mut stream) {
            Ok(statement) => statements.push(statement),
            Err(err) => {
                errors.push(err);
                break;
            }
        }
    }

    for e in errors.iter().clone() {
        eprint!("{}", e)
    }

    (statements, errors)
}

fn block(tokens: &mut TokenStream) -> Result<Statement, Error> {
    if tokens.match_advance(&Token::LeftBrace) {
        let mut statements: Vec<Statement> = Vec::new();
        while !tokens.peek_is(&Token::RightBrace) {
            if tokens.is_at_end() {
                tokens.consume(&Token::RightBrace, "Expected } to close block")?;
            }
            let expr = block(tokens)?;
            statements.push(expr);
        }

        tokens.consume(&Token::RightBrace, "Expected } to close block")?;
        return Ok(Statement::Block(statements));
    }

    statement(tokens)
}

fn statement(tokens: &mut TokenStream) -> Result<Statement, Error> {
    if tokens.match_advance(&Token::Print) {
        return print_statement(tokens);
    }

    if tokens.match_advance(&Token::If) {
        return if_statement(tokens);
    }

    if tokens.match_advance(&Token::Var) {
        return var_declaration(tokens);
    }

    if tokens.match_advance(&Token::Return) {
        return return_statement(tokens);
    }

    if tokens.match_advance(&Token::While) {
        return while_statement(tokens);
    }

    if tokens.match_advance(&Token::For) {
        return for_statement(tokens);
    }

    if tokens.match_advance(&Token::Fun) {
        return fn_statement(tokens);
    }

    let expr = expression(tokens)?;
    tokens.consume(&Token::SemiColon, "Expected ';' after expression.")?;
    Ok(Statement::Expression(expr))
}

fn return_statement(tokens: &mut TokenStream) -> Result<Statement, Error> {
    let mut value = Expression::Primary(Primary::Nil);

    if !tokens.peek_is(&Token::SemiColon) {
        value = expression(tokens)?
    }

    tokens.consume(&Token::SemiColon, "Expected ';' after return.")?;
    Ok(Statement::Return(value))
}

fn fn_statement(tokens: &mut TokenStream) -> Result<Statement, Error> {
    let identifier = tokens.consume_identifier("Function name expected")?;
    tokens.consume(&Token::LeftParen, "Error at fn expected '('")?;

    let mut params: Vec<Token> = Vec::new();

    while tokens.peek().unwrap_or(&Token::Unknown) != &Token::RightParen {
        if !params.is_empty() {
            tokens.consume(&Token::Comma, "Expected comma splitting function arguments")?;
        }
        let token = tokens.advance().unwrap();
        params.push(token);
    }

    tokens.consume(&Token::RightParen, "Error at fn expected ')'")?;

    if !tokens.peek_is(&Token::LeftBrace) {
        return Err(Error::ParseError(
            tokens.current_line(),
            "Expected '{' after function declaration".to_string(),
        ));
    }

    let block = block(tokens)?;

    Ok(Statement::Fn(identifier, params, Box::new(block)))
}

fn for_statement(tokens: &mut TokenStream) -> Result<Statement, Error> {
    tokens.consume(&Token::LeftParen, "Expected '(' after statement.")?;
    let _ = tokens.consume(&Token::Var, "Error at var expected declaration after for.");

    let statement = match var_declaration(tokens) {
        Ok(statement) => Some(Box::new(statement)),
        Err(_) => {
            tokens.consume(&Token::SemiColon, "Expected ';'.")?;
            None
        }
    };

    let check = match matches!(
        tokens.peek().unwrap_or(&Token::Unknown),
        Token::Identifier(_)
    ) {
        true => {
            let expr = expression(tokens)?;
            tokens.consume(&Token::SemiColon, "Expected ';'.")?;
            Some(expr)
        }
        false => None,
    };

    let increment = match matches!(
        tokens.peek().unwrap_or(&Token::Unknown),
        Token::Identifier(_)
    ) {
        true => Some(expression(tokens)?),
        false => None,
    };

    tokens.consume(&Token::RightParen, "Expected ')' after conditional.")?;

    let block = block(tokens)?;

    Ok(Statement::For(statement, check, increment, Box::new(block)))
}

fn while_statement(tokens: &mut TokenStream) -> Result<Statement, Error> {
    let expr = conditional_expression(tokens)?;
    let statement = block(tokens)?;

    Ok(Statement::While(expr, Box::new(statement)))
}

fn if_statement(tokens: &mut TokenStream) -> Result<Statement, Error> {
    let expr = conditional_expression(tokens)?;
    let if_stmt = block(tokens)?;

    let else_stmd = match tokens.match_advance(&Token::Else) {
        true => Some(Box::new(block(tokens)?)),
        false => None,
    };

    Ok(Statement::IfElse(expr, Box::new(if_stmt), else_stmd))
}

fn conditional_expression(tokens: &mut TokenStream) -> Result<Expression, Error> {
    tokens.consume(&Token::LeftParen, "Expected '(' after statement.")?;
    let expr = expression(tokens)?;
    tokens.consume(&Token::RightParen, "Expected ')' after conditional.")?;
    Ok(expr)
}

fn print_statement(tokens: &mut TokenStream) -> Result<Statement, Error> {
    let expr = expression(tokens)?;
    tokens.consume(&Token::SemiColon, "Expected ';' after value.")?;
    Ok(Statement::Print(expr))
}

fn var_declaration(tokens: &mut TokenStream) -> Result<Statement, Error> {
    let name = tokens.consume_identifier("Expected variable name.")?;

    let initializer = if tokens.match_advance(&Token::Equal) {
        expression(tokens)?
    } else {
        Expression::Primary(Primary::Nil)
    };

    tokens.consume(
        &Token::SemiColon,
        "Expected ';' after variable declaration.",
    )?;

    Ok(Statement::Declaration(name, initializer))
}

fn expression(tokens: &mut TokenStream) -> Result<Expression, Error> {
    assignment(tokens)
}

fn assignment(tokens: &mut TokenStream) -> Result<Expression, Error> {
    let left = logical_or(tokens)?;

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
            _ => {
                return Err(Error::ParseError(
                    tokens.current_line(),
                    "Invalid assignment".to_string(),
                ));
            }
        }
    }

    Ok(left)
}

fn logical_or(tokens: &mut TokenStream) -> Result<Expression, Error> {
    let mut expr = logical_and(tokens)?;

    while tokens.peek_is(&Token::Or) {
        tokens.advance();
        let right = logical_and(tokens)?;

        expr = Expression::Binary(Box::new(expr), Operator::Or, Box::new(right));
    }

    Ok(expr)
}

fn logical_and(tokens: &mut TokenStream) -> Result<Expression, Error> {
    let mut expr = equality(tokens)?;

    while tokens.peek_is(&Token::And) {
        tokens.advance();
        let right = equality(tokens)?;

        expr = Expression::Binary(Box::new(expr), Operator::And, Box::new(right));
    }

    Ok(expr)
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
        Token::Identifier(identifier) => {
            if tokens.peek_is(&Token::LeftParen) {
                return parse_function_call(identifier, tokens);
            }

            Ok(Expression::Primary(Primary::Identifier(identifier)))
        }

        Token::LeftParen => {
            let expr_inside = expression(tokens)?;
            if tokens.match_advance(&Token::RightParen) {
                Ok(Expression::Primary(Primary::Grouping(Box::new(
                    expr_inside,
                ))))
            } else {
                let error = Error::ParseError(
                    tokens.current_line(),
                    "Expected ')' after expression.".to_string(),
                );
                Err(error)
            }
        }

        _ => {
            let error =
                Error::ParseError(tokens.current_line(), format!("Unknown token {}", token));
            Err(error)
        }
    }
}

fn parse_function_call(identifier: String, tokens: &mut TokenStream) -> Result<Expression, Error> {
    tokens.consume(&Token::LeftParen, "Expected '(' after function name.")?;
    let params = get_params(tokens)?;
    tokens.consume(&Token::RightParen, "Expected ')' after arguments.")?;

    Ok(Expression::Primary(Primary::Function(identifier, params)))
}

fn get_params(tokens: &mut TokenStream) -> Result<Vec<Expression>, Error> {
    let mut params: Vec<Expression> = Vec::new();

    while tokens.peek().unwrap_or(&Token::Unknown) != &Token::RightParen {
        if !params.is_empty() {
            tokens.consume(&Token::Comma, "Expected comma splitting function arguments")?;
        }
        let param = expression(tokens)?;
        params.push(param);
    }

    Ok(params)
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
