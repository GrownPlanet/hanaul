use std::fs;

mod lexer;

fn main() {
    let program = fs::read_to_string("testprogram.han").unwrap();

    let tokens = lexer::lex(program);

    for token in tokens {
        println!("{:?}", token);
    }
}
