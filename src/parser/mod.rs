use std::collections::HashMap;

use crate::lexer::EToken;

use crate::lexer::{Lexer, Token};

#[derive(Debug, PartialEq)]
pub enum Node {
    Element(HtmlElement), // tags
    Text(String),         // Text node
}

impl Clone for Node {
    fn clone(&self) -> Self {
        self.clone()
    }
}

#[derive(Debug, PartialEq)]
pub struct HtmlElement {
    pub tag: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct Parser {
    pub tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(template: String) -> Self {
        Self {
            tokens: Lexer::new(template).tokenize(),
            cursor: 0,
        }
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }

    fn next_token(&mut self) -> Option<&Token> {
        self.cursor += 1;
        self.current_token()
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.cursor + 1)
    }

    pub fn expect_and_consume_token(&mut self, expected: EToken) -> Result<(), String> {
        if let Some(token) = self.current_token() {
            if token.token == expected {
                self.next_token();
                Ok(())
            } else {
                Err(token.format_unexpected_err_with_expected(expected))
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }

    pub fn parse(&mut self) -> Result<Node, String> {
        let token: Option<Token> = match self.current_token() {
            Some(token) => Some(token.clone()),
            None => None,
        };
        match token {
            Some(token) => {
                let result_node = match &token.token {
                    EToken::TEXT(string_value) => {
                        self.next_token();
                        Ok(Node::Text(string_value.to_string()))
                    }
                    _ => self.parse_block(),
                };

                result_node
            }
            None => Err("Unexpected error".to_string()),
        }
    }
    pub fn parse_block(&mut self) -> Result<Node, String> {
        let _ = self.expect_and_consume_token(EToken::LESSTHAN); // First get rid of the symbol
        let current_token = self.current_token();
        // Parse the tag name
        let tag_name = if let Some(Token {
            token: EToken::TEXT(name),
            ..
        }) = current_token
        {
            name.clone()
        } else {
            return Err(current_token.unwrap().format_unexpected_err());
        };
        self.next_token();

        let mut attributes = HashMap::new();

        loop {
            let token = match self.current_token() {
                Some(token) => token.clone(),
                None => break,
            };

            match &token.token {
                EToken::TEXT(attr_name) => {
                    self.next_token();
                    self.expect_and_consume_token(EToken::EQUAL_SYMBOL)?; // Expect '='

                    if let Some(Token {
                        token: EToken::APOSTROPHE,
                        ..
                    }) = self.current_token()
                    {
                        self.next_token();

                        // if let Some(Token {
                        //     token: EToken::TEXT(attr_value),
                        //     ..
                        // }) = self.current_token()
                        // {
                        //     attributes.insert(attr_name.clone(), attr_value.clone());
                        //     self.next_token();
                        // }

                        let mut attr_full_value = String::new();

                        while let Some(Token {
                            token: EToken::TEXT(attr_value),
                            ..
                        }) = self.current_token()
                        {
                            attr_full_value += &format!(" {}",attr_value).to_string();
                            self.next_token();
                        }
                        attributes.insert(attr_name.clone(), attr_full_value.clone());

                        self.expect_and_consume_token(EToken::APOSTROPHE)?;
                    } else {
                        return Err("Expected an attribute value".to_string());
                    }
                }
                EToken::GREATERTHAN => break,
                // Error.
                _ => return Err(token.format_unexpected_err()),
            }
        }

        self.expect_and_consume_token(EToken::GREATERTHAN)?; // Consume '>'

        // Parse children (sub-elements or text content)
        let mut children = Vec::new();
        while let Some(token) = self.current_token() {
            match &token.token {
                EToken::LESS_THAN_SLASH => {
                    // End of the element
                    self.next_token(); // Consume '</'
                    self.expect_and_consume_token(EToken::TEXT(tag_name.clone()))?;
                    self.expect_and_consume_token(EToken::GREATERTHAN)?; // Consume '>'
                    return Ok(Node::Element(HtmlElement {
                        tag: tag_name,
                        attributes,
                        children,
                    }));
                }
                _ => {
                    // Parse a child node (could be text or another element)
                    children.push(self.parse()?);
                }
            }
        }
        // In case the tag wasn't closed
        Err(format!("Unclosed tag: <{}>", tag_name))
    }
}

#[cfg(test)]
mod parser_tests {

    use crate::{lexer::EToken, parser::Parser};

    #[test]
    fn test_parser_utils() {
        let input = "<div>Hello</div>".to_string();
        let mut parser = Parser::new(input.clone());
        let len = parser.tokens.len();
        // Print the tokens and the first token from the parser
        for token in parser.tokens.iter() {
            println!("{:?}", token);
        }

        match parser.expect_and_consume_token(EToken::GREATERTHAN) {
            Ok(_) => println!("Successfully consumed '>'"),
            Err(err) => eprintln!("Error: {}", err),
        }
        println!("Current token: {:?}", parser.current_token());
        println!("Current token: {:?}", parser.peek_token());
        assert_eq!(parser.current_token(), Some(&parser.tokens[0]));
        assert_eq!(parser.peek_token(), Some(&parser.tokens[1]));
        // let next_token = parser.next_token();
        // println!("Next token: {:?}", next_token);

        let node = parser.parse();
        println!("Result node after parsing: {:?}", node)
    }
}
