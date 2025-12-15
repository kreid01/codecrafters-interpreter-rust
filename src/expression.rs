use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Expression {
    Binary(Box<Expression>, Operator, Box<Expression>),
    Unary(Unary, Box<Expression>),
    Primary(Primary),
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Primary(literal) => {
                write!(f, "{}", literal)
            }
            Expression::Unary(unary, expr) => {
                write!(f, "({} {})", unary, expr)
            }
            Expression::Binary(left, op, right) => {
                write!(f, "({} {} {})", op, left, right)
            }
        }
    }
}

#[derive(Debug)]
pub enum Primary {
    Number(String),
    String(String),
    True,
    False,
    Nil,
    Grouping(Box<Expression>),
}

impl Display for Primary {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        let literal = match self {
            Primary::Number(literal) => literal.to_string(),
            Primary::String(literal) => literal.to_string(),
            Primary::True => "true".to_string(),
            Primary::False => "false".to_string(),
            Primary::Nil => "nil".to_string(),
            Primary::Grouping(expr) => format!("(group {})", expr),
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
            Operator::GreaterEqual => ">=",
            Operator::Division => "/",
            Operator::Star => "*",
            Operator::Plus => "+",
            Operator::Minus => "-",
        };

        write!(fmt, "{}", operator)
    }
}
