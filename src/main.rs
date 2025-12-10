use std::env;
use std::io::stdin;

use crate::parser::parse;
use crate::tokenizer::{print_tokens, tokenize};

mod expression;
mod parser;
mod tokenizer;
mod tokens;
mod utils;

// USE
// mem & boxing
// no unwraps

fn main() {
    // let mut input = String::new();
    // let stdin = stdin();
    // stdin.read_line(&mut input).unwrap();
    // println!("{}", input);
    //
    // parse(&input);
    //
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let (output, errors) = tokenize(filename);
            print_tokens(output, errors);
        }
        "parse" => parse(filename),
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
