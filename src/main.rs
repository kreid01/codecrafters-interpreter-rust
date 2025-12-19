use std::env;
use std::io::stdin;

use crate::evaluator::evaluate;
use crate::parser::parse;
use crate::run::run;
use crate::tokenizer::tokenize;
use crate::utils::{print_output, print_tokenizer_output};

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
    // execute(&input);

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let (tokens, errors) = tokenize(filename);
            print_tokenizer_output(tokens, errors);
        }
        "parse" => {
            let (ast, errors) = parse(filename);
            print_output(ast.iter().map(|x| x.to_string()).collect(), errors, 65);
        }
        "evaluate" => {
            // let (statements, _) = parse(filename);
            // let (output, errors) = evaluate(statements);
            // print_output(output.iter().map(|x| x.to_string()).collect(), errors, 65);
        }
        "run" => run(filename),
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
