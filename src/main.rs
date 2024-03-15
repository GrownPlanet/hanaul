use std::fs;

mod lexer;
mod parser;
mod token;

fn main() {
    let program = fs::read_to_string("testprogram.han").unwrap();

    let lexer = lexer::Lexer::new(program);
    let mut parser = parser::Parser::new(lexer);
    parser.program();

    println!("parsing complete");
}
