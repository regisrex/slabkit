pub struct InputReader {
    pub input: String,
    pub index: usize,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug)]

pub struct Token {
    line: usize,
    column: usize,
    token: EToken,
}

#[derive(Debug)]
pub enum EToken {
    LESSTHAN,                 // "<"
    GREATERTHAN,              // ">"
    LESS_THAN_SLASH,          // "</"
    PLACEHOLDER_START_SYMBOL, // "!{"
    PLACEHOLDER_END_SYMBOL,   // "}!"
    EQUAL_SYMBOL,             // "="
    APOSTROPHE,               // "\'" | "\""
    ATTRKEY(String),          // id | class
    ATTRVALUE(String),        // "value"
    TEXT(String),             // "Hello World"
    CHAR(char),
    DOT,
    EOF,
}

#[derive(Debug)]

pub struct Elt {
    pub tag: String,
    pub children: Vec<Node>,
}

#[derive(Debug)]
pub enum Node {
    Element(Elt),
    Text(String),
}

impl InputReader {
    pub fn new(input: String) -> Self {
        Self {
            input,
            index: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn next_char(&mut self) -> Option<char> {
        let ch = self.input.chars().nth(self.index);
        match ch {
            Some('\n') => {
                self.col = 0;
                self.line += 1;
            }
            _ => {}
        }
        self.index += 1;
        self.col += 1;
        ch
    }

    pub fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.index)
    }

    pub fn consume_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }
    pub fn token(&mut self, tokenType: EToken) -> Token {
        Token {
            column: self.col,
            line: self.line,
            token: tokenType,
        }
    }
}

pub struct Lexer {
    reader: InputReader,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            reader: InputReader::new(input),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        loop {
            let ch = self.reader.next_char();
            if ch.is_none() {
                break;
            }
            match ch.unwrap() {
                '<' => {
                    let next_char = self.reader.peek_char();
                    match next_char {
                        Some('/') => {
                            tokens.push(self.reader.token(EToken::LESS_THAN_SLASH));
                            self.reader.next_char();
                        }
                        _ => tokens.push(self.reader.token(EToken::LESSTHAN)),
                    }
                }
                '>' => tokens.push(self.reader.token(EToken::GREATERTHAN)),
                '!' => {
                    let next_char = self.reader.peek_char();
                    match next_char {
                        Some('{') => {
                            self.reader.next_char();
                            tokens.push(self.reader.token(EToken::PLACEHOLDER_START_SYMBOL));
                        }
                        _ => tokens.push(
                            self.reader
                                .token(EToken::TEXT(next_char.unwrap().to_string())),
                        ),
                    }
                }
                '}' => {
                    let next_char = self.reader.peek_char();
                    match next_char {
                        Some('!') => {
                            self.reader.next_char();
                            tokens.push(self.reader.token(EToken::PLACEHOLDER_END_SYMBOL));
                        }
                        _ => tokens.push(
                            self.reader
                                .token(EToken::TEXT(next_char.unwrap().to_string())),
                        ),
                    }
                }
                '=' => tokens.push(self.reader.token(EToken::EQUAL_SYMBOL)),
                '.' => tokens.push(self.reader.token(EToken::DOT)),
                '\'' | '\"' => tokens.push(self.reader.token(EToken::APOSTROPHE)),
                _ => {
                    if ch.is_none() {
                        break;
                    }
                    if ch.unwrap().is_whitespace() {
                        self.reader.consume_whitespace();
                        continue;
                    }
                    let mut char_sequence = String::from(ch.unwrap());
                    loop {
                        if let Some(next_char) = self.reader.peek_char() {
                            if next_char.is_alphanumeric() || next_char == '-' {
                                char_sequence.push(next_char);
                                self.reader.next_char();
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    tokens.push(self.reader.token(EToken::TEXT(char_sequence)));
                }
            }
        }

        tokens
    }

}
