#![allow(unused_variables)]
use std::fmt::Display;
use std::fs;
use std::{env, fmt};

enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Unknown,
}

impl Display for Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        let token = match *self {
            Self::LeftParen => "LEFT_PAREN ( null",
            Self::RightParen => "RIGHT_PAREN ) null",
            Self::LeftBrace => "LEFT_BRACE { null",
            Self::RightBrace => "RIGHT_BRACE } null",
            Self::Unknown => "",
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
                _ => Token::Unknown,
            };
            output.push(token);
        }
    }

    for token in output {
        println!("{}", token)
    }

    println!("EOF  null");
}
