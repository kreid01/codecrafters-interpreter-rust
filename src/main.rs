#![allow(unused_variables)]
use std::fmt::Display;
use std::{env, fmt};
use std::{fs, process};

enum Token {
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

const RESERVED_KEYWORDS: [&str; 16] = [
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

// USE
// impl
// traits
// mem & boxing
// no unwraps

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => tokenize(command, filename),
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn tokenize(command: &str, filename: &str) {
    eprintln!("Logs from your program will appear here!");

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    let mut output: Vec<Token> = Vec::new();
    let mut errors: Vec<Token> = Vec::new();

    if !file_contents.is_empty() {
        for (i, line) in file_contents.lines().enumerate() {
            let mut tokens = line.chars().peekable();
            let line_number = i + 1;

            while let Some(token) = tokens.next() {
                let token = match token {
                    '(' => Token::LeftParen,
                    ')' => Token::RightParen,
                    '{' => Token::LeftBrace,
                    '}' => Token::RightBrace,
                    '.' => Token::Dot,
                    ',' => Token::Comma,
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '*' => Token::Star,
                    ';' => Token::SemiColon,
                    '=' => get_equal_token(&mut tokens, Token::EqualEqual, Token::Equal),
                    '!' => get_equal_token(&mut tokens, Token::BangEqual, Token::Bang),
                    '>' => get_equal_token(&mut tokens, Token::GreaterEqual, Token::Greater),
                    '<' => get_equal_token(&mut tokens, Token::LessEqual, Token::Less),
                    '/' => match tokens.peek().copied() {
                        Some('/') => break,
                        _ => Token::Division,
                    },
                    ' ' | '\n' | '\t' => {
                        continue;
                    }
                    '"' => get_string_token(&mut tokens, line_number),
                    number if token.is_numeric() => {
                        get_numeric_token(&mut tokens, token, line_number)
                    }
                    identifier if token.is_alphabetic() || token == '_' => {
                        get_identifier(&mut tokens, token, line_number)
                    }
                    _ => Token::Error(token, line_number),
                };

                match token {
                    Token::ErrorString(_, _) | Token::Error(_, _) => {
                        errors.push(token);
                    }
                    _ => {
                        output.push(token);
                    }
                }
            }
        }
    }

    let has_errors = !errors.is_empty();

    for token in errors {
        eprintln!("{}", token);
    }

    for token in output {
        println!("{}", token);
    }

    println!("EOF  null");

    if has_errors {
        process::exit(65)
    }
}

fn get_identifier(
    tokens: &mut std::iter::Peekable<std::str::Chars<'_>>,
    token: char,
    line_number: usize,
) -> Token {
    let mut word = token.to_string();

    while let Some(next) = tokens.peek().cloned() {
        match next {
            char if next.is_alphanumeric() || next == '_' => {
                tokens.next();
                word.push(char);
            }
            _ => return identifier_or_keyword(word),
        }
    }

    identifier_or_keyword(word)
}

fn identifier_or_keyword(word: String) -> Token {
    for reserved in RESERVED_KEYWORDS {
        if word == reserved {
            return Token::Reserved(word);
        }
    }

    Token::Identifier(word)
}

fn get_numeric_token(
    tokens: &mut std::iter::Peekable<std::str::Chars<'_>>,
    token: char,
    line: usize,
) -> Token {
    let mut number = token.to_string();
    let mut decimal = false;

    while let Some(next) = tokens.peek().cloned() {
        match next {
            '.' => {
                if decimal {
                    return Token::ErrorString(number, 1);
                }

                decimal = true;
                tokens.next();
                number.push(next);
            }
            c if next.is_numeric() => {
                tokens.next();
                number.push(c);
            }
            _ => return parse_number(number, line),
        }
    }

    parse_number(number, line)
}

fn parse_number(string: String, line: usize) -> Token {
    let number = string.parse::<f64>();
    let decimal = string.contains('.');
    match number {
        Ok(number) => Token::Number(string, number),
        Err(_) => Token::ErrorString(string, line),
    }
}

fn get_string_token(tokens: &mut std::iter::Peekable<std::str::Chars<'_>>, line: usize) -> Token {
    let mut string = String::new();
    while let Some(next) = tokens.peek().cloned() {
        match next {
            '"' => {
                tokens.next();
                return Token::String(string);
            }
            c => {
                tokens.next();
                string.push(c);
            }
        }
    }

    Token::ErrorString(string, line)
}

fn get_equal_token(
    tokens: &mut std::iter::Peekable<std::str::Chars<'_>>,
    if_equal_token: Token,
    token: Token,
) -> Token {
    match tokens.peek().copied() {
        Some('=') => {
            tokens.next();
            if_equal_token
        }
        _ => token,
    }
}
