#![allow(unused_variables)]
use std::fmt::Display;
use std::io::{Write, stderr};
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
