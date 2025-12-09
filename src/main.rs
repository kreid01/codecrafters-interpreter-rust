#![allow(unused_variables)]
use std::fmt::Display;
use std::io::{Write, stderr};
use std::iter::Peekable;
use std::str::Chars;
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
    Error(char, u32),
}

impl Display for Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        let token = match *self {
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
            Self::Error(char, line) => {
                format!("[line {}] Error: Unexpected character: {}", &line, &char)
            }
        };

        write!(fmt, "{}", token)
    }
}

// USE
// impl
// traits
// mom & boxing
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

    let mut tokens = file_contents.chars().peekable();
    let mut output: Vec<Token> = Vec::new();

    if !file_contents.is_empty() {
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
                '=' => check_equal_token(&mut tokens, Token::EqualEqual, Token::Equal),
                '!' => check_equal_token(&mut tokens, Token::BangEqual, Token::Bang),
                '>' => check_equal_token(&mut tokens, Token::GreaterEqual, Token::Greater),
                '<' => check_equal_token(&mut tokens, Token::LessEqual, Token::Less),
                _ => Token::Error(token, 1),
            };
            output.push(token);
        }
    }

    let has_error = output.iter().any(|x| matches!(x, Token::Error(_, _)));
    let stderr = stderr();

    for token in output {
        match token {
            Token::Error(_, _) => eprintln!("{}", token),
            _ => println!("{}", token),
        }
    }

    println!("EOF  null");

    if has_error {
        process::exit(65)
    }
}

fn check_equal_token(
    tokens: &mut std::iter::Peekable<std::str::Chars<'_>>,
    if_equal_token: Token,
    token: Token,
) -> Token {
    if let Some(next) = tokens.peek()
        && *next == '='
    {
        tokens.next();
        if_equal_token
    } else {
        token
    }
}
