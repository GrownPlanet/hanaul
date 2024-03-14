use crate::token::{Token, TokenType};

pub struct Parser {
    current_token: Token,
    peek_token: Token,
    tokens: Vec<Token>,
}

impl Parser {
    fn check_token(&self, kind: TokenType) -> bool {
        self.current_token.kind() == kind
    }

    fn die(message: String) -> ! {
        println!("Error while parsing: {}", message);
        std::process::exit(1);
    }
}
