#[derive(Debug)]
pub struct Token {
    text: String,
    kind: TokenType,
}

impl Token {
    pub fn new(text: String, kind: TokenType) -> Self {
        Self { text, kind }
    }

    pub fn kind(&self) -> TokenType {
        self.kind
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[rustfmt::skip]
pub enum TokenType {
    Eof, Newline, Int, Float, Ident, Sting, 
    // keywords
    Label, Goto, Print, Input, Let, If, Then, Endif, While, Repeat, EndWhile,
    // operators
    Eq, Plus, Minus, Asterisk, Slash, EqEq, NotEq, Lt, LtEq, Gt, GtEq
}
