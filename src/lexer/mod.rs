pub struct InputReader {
    pub input: String,
    pub index: usize,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq, Clone)]

pub struct Token {
    pub line: usize,
    pub column: usize,
    pub token: EToken,
}

impl Token {
    pub fn format_unexpected_err(&self) -> String {
        format!(
            "Unexpected token {} at line {}, column {}",
            self.token.to_symbol(),
            self.line,
            self.column
        )
    }

    pub fn format_unexpected_err_with_expected(&self, expected: EToken) -> String {
        format!(
            "Unexpected token {} at line {}, column {}, expected : {}",
            self.token.to_symbol(),
            self.line,
            self.column,
            expected.to_symbol()
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum EToken {
    LESSTHAN,                 // "<"
    GREATERTHAN,              // ">"
    LESS_THAN_SLASH,          // "</"
    PLACEHOLDER_START_SYMBOL, // "!{"
    PLACEHOLDER_END_SYMBOL,   // "}!"
    EQUAL_SYMBOL,             // "="
    APOSTROPHE,               // "\'" | "\""
    TEXT(String),             // "Hello World"
    DOT,
}

impl EToken {
    pub fn to_symbol(&self) -> String {
        match self {
            EToken::LESSTHAN => "<".to_string(),
            EToken::GREATERTHAN => ">".to_string(),
            EToken::LESS_THAN_SLASH => "</".to_string(),
            EToken::PLACEHOLDER_START_SYMBOL => "!{".to_string(),
            EToken::PLACEHOLDER_END_SYMBOL => "}!".to_string(),
            EToken::EQUAL_SYMBOL => "=".to_string(),
            EToken::APOSTROPHE => "\"".to_string(), // Or use '\''
            EToken::DOT => ".".to_string(),
            EToken::TEXT(content) => content.clone(),
        }
    }
}

#[derive(Debug)]

pub struct Elt {
    pub tag: String,
    pub children: Vec<LexicNode>,
}

#[derive(Debug)]
pub enum LexicNode {
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

    pub fn last_char(&mut self) -> Option<char> {
        let current_index = self.index;
        self.input.chars().nth(current_index - 1)
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
            if ch.unwrap().is_whitespace() {
                self.reader.consume_whitespace();
                continue;
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
                // '!' => {
                //     let next_char = self.reader.peek_char();
                //     match next_char {
                //         Some('{') => {
                //             self.reader.next_char();
                //             tokens.push(self.reader.token(EToken::PLACEHOLDER_START_SYMBOL));
                //         }
                //         _ => tokens.push(
                //             self.reader
                //                 .token(EToken::TEXT(next_char.unwrap().to_string())),
                //         ),
                //     }
                // }
                // '}' => {
                //     let next_char = self.reader.peek_char();
                //     match next_char {
                //         Some('!') => {
                //             self.reader.next_char();
                //             tokens.push(self.reader.token(EToken::PLACEHOLDER_END_SYMBOL));
                //         }
                //         _ => tokens.push(
                //             self.reader
                //                 .token(EToken::TEXT(next_char.unwrap().to_string())),
                //         ),
                //     }
                // }
                '=' => tokens.push(self.reader.token(EToken::EQUAL_SYMBOL)),
                // '.' => tokens.push(self.reader.token(EToken::DOT)),
                '\'' | '\"' => tokens.push(self.reader.token(EToken::APOSTROPHE)),
                '\n' => {
                    self.reader.next_char();
                }
                _ => {
                    if ch.is_none() {
                        break;
                    }
                    let mut char_sequence = String::from(ch.unwrap());
                    loop {
                        if let Some(next_char) = self.reader.peek_char() {
                            if next_char.is_alphanumeric()
                                || ['}', '!', '{', '@', '-', '_', ':', '.', '&', ';', ',']
                                    .contains(&next_char)
                            // || next_char.is_whitespace()
                            {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder_tokenization() {
        let input = "!{placeholder}! = value";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token {
                    token: EToken::TEXT("!{placeholder}!".to_string()),
                    line: 1,
                    column: 16
                },
                Token {
                    token: EToken::EQUAL_SYMBOL,
                    line: 1,
                    column: 18
                },
                Token {
                    token: EToken::TEXT("value".to_string()),
                    line: 1,
                    column: 24
                }
            ]
        );
    }
}
