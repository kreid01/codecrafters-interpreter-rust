use crate::enums::statement::Statement;
use crate::evaluator::evaluate;
use crate::parser::parse;

pub fn run(filename: &str) {
    let (statements, _) = parse(filename);

    for statement in statements {
        match statement {
            Statement::Print(expression) => {
                let eval = evaluate(expression);
                println!("{}", eval);
            }
        }
        println!("{}", statement)
    }
}
