use crate::utils::get_file_contents;

pub fn parse(command: &str, filename: &str) {
    let tokens: Vec<String> = get_file_contents(filename)
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();

    for token in tokens {
        println!("{}", token)
    }
}
