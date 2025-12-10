use std::fmt::{self, Display};

pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Dot,
    Comma,
    Plus,
    Minus,
    Star,
    SemiColon,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Division,
    String(String),
    Identifier(String),
    Number(String, f64),
    Error(char, usize),
    ErrorString(String, usize),
    Reserved(String),
}

pub const RESERVED_KEYWORDS: [&str; 16] = [
    "and", "class", "else", "false", "for", "fun", "if", "nil", "or", "print", "return", "super",
    "this", "true", "var", "while",
];

impl Display for Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        let token = match self {
            Self::LeftParen => "LEFT_PAREN ( null".to_string(),
            Self::RightParen => "RIGHT_PAREN ) null".to_string(),
            Self::LeftBrace => "LEFT_BRACE { null".to_string(),
            Self::RightBrace => "RIGHT_BRACE } null".to_string(),
            Self::Comma => "COMMA , null".to_string(),
            Self::Dot => "DOT . null".to_string(),
            Self::Plus => "PLUS + null".to_string(),
            Self::Minus => "MINUS - null".to_string(),
            Self::Star => "STAR * null".to_string(),
            Self::SemiColon => "SEMICOLON ; null".to_string(),
            Self::Equal => "EQUAL = null".to_string(),
            Self::EqualEqual => "EQUAL_EQUAL == null".to_string(),
            Self::BangEqual => "BANG_EQUAL != null".to_string(),
            Self::Bang => "BANG ! null".to_string(),
            Self::LessEqual => "LESS_EQUAL <= null".to_string(),
            Self::Less => "LESS < null".to_string(),
            Self::GreaterEqual => "GREATER_EQUAL >= null".to_string(),
            Self::Greater => "GREATER > null".to_string(),
            Self::Division => "SLASH / null".to_string(),
            Self::Identifier(identifier) => {
                format!("IDENTIFIER {} null", &identifier)
            }
            Self::String(string) => {
                format!("STRING \"{}\" {}", &string, &string)
            }
            Self::Number(string, number) => {
                if number.to_string().contains('.') {
                    format!("NUMBER {} {}", &string, &number)
                } else {
                    format!("NUMBER {} {}.0", &string, &number)
                }
            }
            Self::Error(char, line) => {
                format!("[line {}] Error: Unexpected character: {}", &line, &char)
            }
            Self::ErrorString(char, line) => {
                format!("[line {}] Error: Unterminated string.", &line)
            }
            Self::Reserved(keyword) => {
                format!("{} {} null", &keyword.to_uppercase(), &keyword)
            }
        };

        write!(fmt, "{}", token)
    }
}
