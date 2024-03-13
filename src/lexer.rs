pub struct Lexer {
    source: String,
    current_char: char,
    current_pos: usize,
}

impl Lexer {
    pub fn new(mut source: String) -> Self {
        source.push('\n');
        Self {
            // should be fine since we just appended a newline to source
            current_char: source.chars().next().unwrap(),
            source,
            current_pos: 0,
        }
    }

    pub fn next_char(&mut self) {
        self.current_pos += 1;

        if self.current_pos > self.source.len() {
            self.current_char = '\0';
        } else {
            self.current_char = self.source.chars().nth(self.current_pos).unwrap();
        }
    }

    pub fn peek(&self) -> char {
        if self.current_pos + 1 >= self.source.len() {
            return '\0';
        } else {
            return self.source.chars().nth(self.current_pos + 1).unwrap();
        }
    }

    pub fn current_char(&self) -> char {
        self.current_char
    }

    pub fn get_token(&self) -> Option<Token> {
        let current_char_str: String = self.current_char.into();
        let token: Option<Token>;

        match self.current_char {
            '+' => token = Some(Token::new(current_char_str, TokenType::Plus)),
            '-' => token = Some(Token::new(current_char_str, TokenType::Minus)),
            '*' => token = Some(Token::new(current_char_str, TokenType::Asterisk)),
            '/' => token = Some(Token::new(current_char_str, TokenType::Slash)),
            '\n' => token = Some(Token::new(current_char_str, TokenType::Newline)),
            '\0' => token = Some(Token::new(current_char_str, TokenType::Eof)),
            _ => token = None,
        }

        token
    }
}

#[derive(Debug)]
pub struct Token {
    text: String,
    kind: TokenType,
}

impl Token {
    pub fn new(text: String, kind: TokenType) -> Self {
        Self { text, kind }
    }
}

#[derive(Debug)]
#[rustfmt::skip]
pub enum TokenType {
    Eof, Newline, Int, Float, Indent, Sting, 
    // keywords
    Label, Goto, Print, Input, Let, If, Then, Endif, While, Repeat, EndWhile,
    // operators
    Eq, Plus, Minus, Asterisk, Slash, EqEq, NoteQ, Lt, Lteq, Gt, GteQ
}
