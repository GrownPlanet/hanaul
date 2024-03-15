/*
=== grammar for haneul ===

program ::= {statement}
statement ::= "PRINT" (expression | string) nl
    | "IF" comparison "THEN" nl {statement} "ENDIF" nl
    | "WHILE" comparison "REPEAT" nl {statement} "ENDWHILE" nl
    | "LABEL" ident nl
    | "GOTO" ident nl
    | "LET" ident "=" expression nl
    | "INPUT" ident nl
comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)+
expression ::= term {( "-" | "+" ) term}
term ::= unary {( "/" | "*" ) unary}
unary ::= ["+" | "-"] primary
primary ::= number | ident
nl ::= '\n'+
*/

use crate::{
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Token::default(),
            peek_token: Token::default(),
        };
        parser.next_token();
        parser.next_token(); // call twice to set the current and the peek token

        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.get_token();
        self.lexer.next_char();
    }

    fn match_token(&mut self, kind: TokenType) {
        if !self.check_token(kind) {
            Self::die(format!["Expected {:?}, got {:?}", kind, self.current_token])
        }
        self.next_token();
    }

    fn check_token(&self, kind: TokenType) -> bool {
        self.current_token.kind() == kind
    }

    fn check_peek(&self, kind: TokenType) -> bool {
        self.peek_token.kind() == kind
    }

    fn die(message: String) -> ! {
        println!("Error while parsing: {}", message);
        std::process::exit(1);
    }

    // grammar
    // program ::= {statement}
    pub fn program(&mut self) {
        println!("PROGRAM");

        while !self.check_token(TokenType::Eof) {
            self.statement();
        }
    }

    fn statement(&mut self) {
        match self.current_token.kind() {
            // "PRINT" (expression | string) nl
            TokenType::Print => {
                println!("STATEMENT-PRINT");
                self.next_token();

                if self.check_token(TokenType::String) {
                    self.next_token();
                } else {
                    self.expression();
                }
            }
            // "IF" comparison "THEN" nl {statement} "ENDIF" nl
            TokenType::If => {
                println!("STATEMENT-IF");
                self.next_token();
                self.comparison();

                self.match_token(TokenType::Then);
                self.nl();

                while !self.check_token(TokenType::Endif) {
                    self.statement();
                }

                self.match_token(TokenType::If);
            }
            // "WHILE" comparison "REPEAT" nl {statement} "ENDWHILE" nl
            TokenType::While => {
                println!("STATEMENT-WHILE");
                self.next_token();
                self.comparison();

                self.match_token(TokenType::Repeat);
                self.nl();

                while !self.check_token(TokenType::EndWhile) {
                    self.statement();
                }

                self.match_token(TokenType::EndWhile);
            }
            // "LABEL" ident nl
            TokenType::Label => {
                println!("STATEMENT-LABEL");
                self.next_token();
                self.match_token(TokenType::Ident);
            }
            // "GOTO" ident nl
            TokenType::Goto => {
                println!("STATEMENT-GOTO");
                self.next_token();
                self.match_token(TokenType::Ident);
            }
            // "LET" ident "=" expression nl
            TokenType::Let => {
                println!("STATEMENT-LET");
                self.next_token();
                self.match_token(TokenType::Ident);
                self.match_token(TokenType::Eq);
                self.expression();
            }
            // "INPUT" ident nl
            TokenType::Input => {
                println!("STATEMENT-INPUT");
                self.next_token();
                self.match_token(TokenType::Input);
            }
            _ => Self::die(format![
                "Invalid statement at: {} ({:?})",
                self.current_token.text(),
                self.current_token.kind()
            ]),
        };

        self.nl();
    }

    fn nl(&mut self) {
        println!("NEWLINE");

        self.match_token(TokenType::Newline);
        while self.check_token(TokenType::Newline) {
            self.next_token();
        }
    }

    fn comparison(&mut self) {
        println!("COMPARISON");

        todo!()
    }

    fn expression(&mut self) {
        println!("EXPRESSION");

        todo!()
    }
}
