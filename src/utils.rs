use std::fmt::Display;
use std::{fs, process};

pub fn get_file_contents(filename: &str) -> String {
    fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        filename.to_string()
    })
}

pub fn if_error_exit(errors: bool, code: i32) {
    if errors {
        process::exit(code);
    }
}

pub fn print<T>(output: Vec<T>)
where
    T: Display,
{
    for x in output {
        println!("{}", x);
    }
}
