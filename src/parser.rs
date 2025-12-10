use std::collections::VecDeque;
use std::fmt::{self, Display};

use crate::tokenizer::tokenize;
use crate::tokens::{AsString, Token};

pub fn parse(filename: &str) {
    let (tokens, errors) = tokenize(filename);
    let mut ast: Vec<Expression> = Vec::new();

    let mut tokens: VecDeque<Token> = VecDeque::from(tokens);

    while let Some(token) = tokens.pop_front() {
        if let Some(expr) = parse_token(token, &mut tokens) {
            ast.push(expr);
        }
    }

    for syntax in &ast {
        println!("{}", syntax);
    }
}

fn parse_token(token: Token, tokens: &mut VecDeque<Token>) -> Option<Expression> {
    match token {
        Token::String(literal) => get_string_literal_expression(literal),
        Token::Number(_, number) => Some(Expression::Literal(Literal::Number(number))),
        Token::LeftParen => get_group_expression(tokens),
        Token::Bang => get_unary_expression(Unary::Bang, Token::Bang, tokens),
        Token::Minus => get_unary_expression(Unary::Minus, Token::Minus, tokens),
        Token::False => Some(Expression::Literal(Literal::False)),
        Token::True => Some(Expression::Literal(Literal::True)),
        _ => get_string_literal_expression(token.literal().to_string()),
    }
}

fn get_group_expression(tokens: &mut VecDeque<Token>) -> Option<Expression> {
    let mut inner: String = "(group ".to_string();
    while let Some(next) = tokens.pop_front() {
        match next {
            Token::RightParen => inner.push(')'),
            token => {
                if let Some(expression) = parse_token(token.to_owned(), tokens) {
                    inner.push_str(&expression.to_string());
                }
            }
        }
    }
    Some(Expression::Literal(Literal::String(inner)))
}

fn get_string_literal_expression(literal: String) -> Option<Expression> {
    Some(Expression::Literal(Literal::String(literal)))
}

fn get_unary_expression(
    unary: Unary,
    token: Token,
    tokens: &mut VecDeque<Token>,
) -> Option<Expression> {
    let expression = match tokens.pop_front() {
        Some(Token::True) => Expression::Unary(unary, Box::new(Expression::Literal(Literal::True))),
        Some(Token::False) => {
            Expression::Unary(unary, Box::new(Expression::Literal(Literal::False)))
        }
        Some(Token::Number(_, literal)) => Expression::Unary(
            unary,
            Box::new(Expression::Literal(Literal::Number(literal))),
        ),
        Some(token) => {
            if let Some(next) = parse_token(token, tokens) {
                Expression::Unary(unary, Box::new(next))
            } else {
                return None;
            }
        }
        None => {
            return None;
        }
    };
    Some(expression)
}

const NUMERIC_OPERATORS: [Token; 4] = [Token::Division, Token::Minus, Token::Star, Token::Plus];

fn parse_numeric_token(literal: String, tokens: &[Token]) -> Option<Expression> {
    while let Some(next) = tokens.iter().next().cloned()
        && (matches!(next, Token::Number(_, _)) || NUMERIC_OPERATORS.contains(&next))
    {
        match next {
            _ => {
                println!("{:?}", next);
            }
        }
    }

    Some(Expression::Literal(Literal::Number(literal)))
}

// output (/ (* 16.0 38.0) 58.0)
// Expression1 =
//

struct AST {
    node: String,
    left: Option<String>,
    right: Option<String>,
}

enum Expression {
    Literal(Literal),
    Grouping(Box<Expression>),
    Unary(Unary, Box<Expression>),
    Binary(Box<Expression>, Operator, Box<Expression>),
    Operator(Operator),
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Literal(literal) => {
                write!(f, "{}", literal)
            }
            Expression::Grouping(expr) => write!(f, "({})", expr),
            Expression::Unary(unary, expr) => {
                write!(f, "({} {})", unary, expr)
            }
            Expression::Binary(left, op, right) => {
                write!(f, "({} {} {})", left, op, right)
            }
            Expression::Operator(operator) => {
                write!(f, "{}", operator)
            }
        }
    }
}

enum Literal {
    Number(String),
    String(String),
    True,
    False,
    Nil,
}

impl Display for Literal {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        let literal = match self {
            Literal::Number(literal) => literal,
            Literal::String(literal) => literal,
            Literal::True => "true",
            Literal::False => "false",
            Literal::Nil => "Nil",
        };

        write!(fmt, "{}", literal)
    }
}

enum Unary {
    Minus,
    Bang,
}

impl Display for Unary {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        let unary = match self {
            Unary::Minus => "-",
            Unary::Bang => "!",
        };

        write!(fmt, "{}", unary)
    }
}

enum Operator {
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Division,
}

impl Display for Operator {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        let operator = match self {
            Operator::Equal => "=",
            Operator::EqualEqual => "==",
            Operator::BangEqual => "!=",
            Operator::Less => "<",
            Operator::LessEqual => "<=",
            Operator::Greater => ">",
            Operator::GreaterEqual => "<=",
            Operator::Division => "/",
        };

        write!(fmt, "{}", operator)
    }
}
