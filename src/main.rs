use std::fs;

use lexer::Lexer;

mod lexer;

fn main() {
    let source = fs::read_to_string("testprogram.han").unwrap();
    let mut lexer = Lexer::new(source);

    while lexer.peek() != '\0' {
        println!("{:?}", lexer.get_token());
        lexer.next_char();
    }
}
