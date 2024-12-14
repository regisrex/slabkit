use std::iter::Skip;

use crate::tokenizer::InputReader;

pub struct SlkAttribute {
    pub key: String,
    pub value: String,
}

pub enum SlkToken {
    LessThan , // <
    GreaterThan ,// >
    Equal, // =
    Apostrophe, // ' , "
    Text(String),
    Identifier(String), // attribute name
    Number(i64),
    SlashGreaterThan, // />
}

pub struct SlkLexer {
    reader: InputReader,
}

impl SlkLexer {
    pub fn new(input: String) -> SlkLexer {
        SlkLexer {
            reader: InputReader::new(input),
        }
    }

    pub fn next_token(&mut self) -> SlkToken {
        let mut token = SlkToken::Text(String::from("//-----"));

        self.reader.consume_space(); // to skip all the starting spaces

        let mut c = self.reader.next_char();

        let token = match c {
            Some('<') => SlkToken::LessThan,
            Some('>') => SlkToken::GreaterThan,
            Some('=') => SlkToken::Equal,
            Some('\'') => SlkToken::Apostrophe,
            Some('"') => SlkToken::Apostrophe,
            Some('/') => { 
                let next = self.reader.next_char();
                match next {
                    Some('>') => SlkToken::SlashGreaterThan,
                    _ => panic!("Invalid token"),
                }
            }
            Some(c) => {
                if c.is_alphabetic() {
                    let mut s = String::new();
                    s.push(c);
                    loop {
                        let next = self.reader.next_char();
                        match next {
                            Some(c) if c.is_alphabetic() => s.push(c),
                            Some(c) if c.is_whitespace() => s.push(c),
                            _ => break,
                        }
                    }
                    SlkToken::Identifier(s)
                } else if c.is_numeric() {
                    let mut s = String::new();
                    s.push(c);
                    loop {
                        let next = self.reader.next_char();
                        match next {
                            Some(c) if c.is_numeric() => s.push(c),
                            Some(c) if c.is_whitespace() => break,
                            _ => break,
                        }
                    }
                    SlkToken::Number(s.parse::<i64>().unwrap())
                } else {
                    panic!("Invalid token");
                }
            }
            None => panic!("Invalid token"),
           
        };

        token
    }
}
