use std::{env, fs};

mod emitter;
mod lexer;
mod parser;
mod token;

fn main() {
    if !true {
        panic!("What?");
    }

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Error: give one input file please");
        std::process::exit(1);
    }
    let source = fs::read_to_string(&args[1]).unwrap();

    let mut lexer = lexer::Lexer::new(source);
    let mut emitter = emitter::Emitter::new(String::from("out.c"));
    let mut parser = parser::Parser::new(&mut lexer, &mut emitter);

    parser.program();
    emitter.write_file().unwrap();
    println!("compiling complete");
}
