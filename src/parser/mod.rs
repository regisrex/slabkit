pub struct InputReader {
    pub input: String,
    pub index: usize,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenTag(String),
    CloseTag,
    Text(String),
    AttributeName(String),
    AttributeValue(String),
    Equals,
    Slash,
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
        Self { input, index: 0 }
    }

    pub fn next_char(&mut self) -> Option<char> {
        let ch = self.input.chars().nth(self.index);
        self.index += 1;
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
    pub fn prev_char(&mut self) -> Option<char> {
        // Get the last character and move the cursor backwards
        self.index = self.index.saturating_sub(1);
        self.input[self.index..self.index + 1].chars().next()
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
}

impl Lexer {
    pub fn peek_token(&mut self) -> Option<Token> {
        let index = self.reader.index;
        let token = self.next_token();
        self.reader.index = index;
        Some(token)
    }
    pub fn next_token(&mut self) -> Token {
        self.reader.consume_whitespace(); // Skip any leading whitespace

        if let Some(ch) = self.reader.next_char() {
            match ch {
                '<' => {
                    if let Some('/') = self.reader.peek_char() {
                        self.reader.next_char(); // Consume '/'
                        Token::CloseTag
                    } else {
                        Token::OpenTag(self.read_tag_name())
                    }
                }
                '>' => Token::CloseTag, // End of a tag
                '=' => Token::Equals,
                '/' => Token::Slash,
                '"' | '\'' => Token::AttributeValue(self.read_quoted_value(ch)),
                '{' => Token::Text(self.read_text(ch)),
                _ if ch.is_alphanumeric() => Token::Text(self.read_text(ch)),

                _ => panic!("Unexpected character: {}", ch),
            }
        } else {
            Token::EOF
        }
    }

    fn read_tag_name(&mut self) -> String {
        let mut tag_name = String::new();
        while let Some(ch) = self.reader.peek_char() {
            if ch.is_alphanumeric() || ch == '-' {
                tag_name.push(ch);
                self.reader.next_char();
            } else {
                break;
            }
        }
        tag_name
    }

    fn read_text(&mut self, first_char: char) -> String {
        let mut text = String::new();
        text.push(first_char);

        while let Some(ch) = self.reader.peek_char() {
            if ch == '<' {
                break; // Stop reading when a tag starts
            }
            text.push(ch);
            self.reader.next_char();
        }
        text
    }

    fn read_quoted_value(&mut self, quote: char) -> String {
        let mut value = String::new();

        while let Some(ch) = self.reader.next_char() {
            if ch == quote {
                break; // End of the quoted value
            }
            value.push(ch);
        }

        value
    }
}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Node {
        self.parse_element()
    }

    fn parse_element(&mut self) -> Node {
        let mut nesting_depth = 1;
        if let Token::OpenTag(tag) = self.lexer.next_token() {
            let mut children = Vec::new();

            loop {
                let token = self.lexer.next_token();
                match token {
                    Token::CloseTag => {
                        nesting_depth -= 1;
                        if nesting_depth == 0 {
                            break;
                        }
                    }
                    Token::OpenTag(_) => {
                        nesting_depth += 1;
                        children.push(self.parse_child_element());
                    }
                    Token::Text(text) => children.push(Node::Text(text)),
                    Token::EOF => panic!("Unexpected EOF"),
                    _ => panic!("Unexpected token: {:?}", token),
                }
            }

            let output_node = Node::Element(Elt { tag, children });
            return output_node;
        } else {
            panic!("Expected a start tag");
        }
    }

    fn parse_child_element(&mut self) -> Node {
        let next_token = self.lexer.peek_token();

        if next_token.is_some() && next_token.unwrap() == Token::OpenTag("".to_string()) {
            return self.parse_element();
        } else {
            self.lexer.reader.index -= 1; // Rewind for the child element
            return self.parse_element();
        }
    }
    // fn parse_element(&mut self) -> Node {
    //     println!("Peeking {:?}", self.lexer.next_token());
    //     if let Token::OpenTag(tag) = self.lexer.next_token() {
    //         let mut children = Vec::new();

    //         loop {
    //             let token = self.lexer.next_token();
    //             match token {
    //                 Token::CloseTag => break,
    //                 Token::OpenTag(_) => {
    //                     self.lexer.reader.index -= 1; // rewind
    //                     children.push(self.parse_element());
    //                 }
    //                 Token::Text(text) => children.push(Node::Text(text)),
    //                 Token::EOF => panic!("Unexpected EOF"),
    //                 _ => panic!("Unexpected token: {:?}", token),
    //             }
    //         }

    //         let output_node = Node::Element( Elt{ tag, children });

    //         println!("Output node {:?} ", output_node);
    //         output_node
    //     } else {
    //         panic!("Expected a start tag");
    //     }
    // }
}
