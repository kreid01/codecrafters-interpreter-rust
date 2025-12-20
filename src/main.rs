use std::io::stdin;
use std::{env, process};

use crate::enums::expression::Expression;
use crate::enums::statement::Statement;
use crate::evaluator::{Value, evaluate};
use crate::parser::parse;
use crate::run::run;
use crate::tokenizer::tokenize;

pub mod enums;
mod evaluator;
mod parser;
mod run;
mod tokenizer;
mod utils;

// mem & boxing
// no unwraps

fn main() {
    // let mut input = String::new();
    // let stdin = stdin();
    // stdin.read_line(&mut input).unwrap();
    // println!("{}", input);
    //
    // run(&input);

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            eprintln!("Logs from your program will appear here!");
            let (tokens, errors) = tokenize(filename);

            let has_errors = !errors.is_empty();

            for e in errors {
                eprintln!("{}", e)
            }

            for token in tokens {
                println!("{}", token)
            }

            println!("EOF  null");

            if has_errors {
                process::exit(65)
            }
        }
        "parse" => {
            let (_, errors) = tokenize(filename);

            if !errors.is_empty() {
                process::exit(65)
            }

            let (expressions, errors) = parse(filename);

            if !errors.is_empty() {
                process::exit(65);
            }

            for e in expressions {
                println!("{}", e)
            }
        }
        "evaluate" => {
            let (expressions, errors) = parse(filename);

            if !errors.is_empty() {
                process::exit(70);
            }

            for e in expressions {
                match evaluate(&e) {
                    Ok(value) => println!("{}", value),
                    Err(_) => process::exit(65),
                }
            }
        }
        "run" => run(filename),
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
