use crate::utils::get_file_contents;

pub fn parse(command: &str, filename: &str) {
    let tokens: Vec<String> = get_file_contents(filename)
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();

    for token in tokens {
        match token.parse::<f32>() {
            Ok(_) => {
                if token.contains('.') {
                    println!("{}", token)
                } else {
                    println!("{}.0", token)
                }
            }
            Err(_) => {
                println!("{}", token)
            }
        }
    }
}
