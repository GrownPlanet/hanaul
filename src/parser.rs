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
            Self::die(format![
                "Expected {:?}, got {}",
                kind,
                self.current_token.text()
            ])
        }
        self.next_token();
    }

    fn check_token(&self, kind: TokenType) -> bool {
        self.current_token.kind() == kind
    }

    fn die(message: String) -> ! {
        println!("Error while parsing: {}", message);
        std::process::exit(1);
    }

    // grammar
    // program ::= {statement}
    pub fn program(&mut self) {
        println!("PROGRAM");

        while self.check_token(TokenType::Newline) {
            self.next_token();
        }

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

                self.match_token(TokenType::Endif);
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

    // comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)+
    fn comparison(&mut self) {
        println!("COMPARISON");

        self.expression();
        if self.is_comparison_operator() {
            self.next_token();
            self.expression();
        } else {
            Self::die(format![
                "Expected comparison at: {}",
                self.current_token.text()
            ]);
        }

        while self.is_comparison_operator() {
            self.next_token();
            self.expression();
        }
    }

    fn is_comparison_operator(&self) -> bool {
        self.check_token(TokenType::Gt)
            || self.check_token(TokenType::GtEq)
            || self.check_token(TokenType::Lt)
            || self.check_token(TokenType::LtEq)
            || self.check_token(TokenType::EqEq)
            || self.check_token(TokenType::NotEq)
    }

    // expression ::= term {( "-" | "+" ) term}
    fn expression(&mut self) {
        println!("EXPRESSION");

        self.term();
        while self.check_token(TokenType::Plus) || self.check_token(TokenType::Minus) {
            self.next_token();
            self.term();
        }
    }

    // term ::= unary {( "/" | "*" ) unary}
    fn term(&mut self) {
        println!("TERM");

        self.unary();
        while self.check_token(TokenType::Asterisk) || self.check_token(TokenType::Slash) {
            self.next_token();
            self.unary();
        }
    }
    // unary ::= ["+" | "-"] primary
    fn unary(&mut self) {
        println!("UNARY");

        if self.check_token(TokenType::Plus) || self.check_token(TokenType::Minus) {
            self.next_token();
        }
        self.primary();
    }
    // primary ::= number | ident
    fn primary(&mut self) {
        println!("PRIMARY ({})", self.current_token.text());

        if self.check_token(TokenType::Float) || self.check_token(TokenType::Int) {
            self.next_token();
        } else if self.check_token(TokenType::Ident) {
            self.next_token();
        } else {
            Self::die(format!["Unexpected token at {}", self.current_token.text()]);
        }
    }
}
