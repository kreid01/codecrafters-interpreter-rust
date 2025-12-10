#![allow(unused_variables)]
use std::env;

use crate::parser::parse;
use crate::tokenizer::tokenize;

mod parser;
mod tokenizer;
mod tokens;
mod utils;

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
        "parse" => parse(command, filename),
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
