use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Expression {
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
            Expression::Grouping(expr) => write!(f, "(group {})", expr),
            Expression::Unary(unary, expr) => {
                write!(f, "({} {})", unary, expr)
            }
            Expression::Binary(left, op, right) => {
                write!(f, "({} {} {})", op, left, right)
            }
            Expression::Operator(operator) => {
                write!(f, "{}", operator)
            }
        }
    }
}

#[derive(Debug)]
pub enum Literal {
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

#[derive(Debug)]
pub enum Unary {
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

#[derive(Debug)]
pub enum Operator {
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Division,
    Star,
    Minus,
    Plus,
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
            Operator::Star => "*",
            Operator::Plus => "+",
            Operator::Minus => "-",
        };

        write!(fmt, "{}", operator)
    }
}
