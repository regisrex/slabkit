pub struct InputReader {
    pub input: String,
    pub index: usize,
}

impl InputReader {
    pub fn new(input: String) -> Self {
        Self { input, index: 0 }
    }

    pub fn peek(&self) -> Option<char> {
        if self.index < self.input.len() {
            self.input.chars().nth(self.index)
        } else {
            None
        }
    }

    pub fn consume_space(&mut self) {
        while let Some(c) = self.next_char() {
            if !c.is_whitespace() {
                self.index -= 1;
                break;
            }
        }
    }
    pub fn next_char(&mut self) -> Option<char> {
        if self.index < self.input.len() {
            let c = self.input.chars().nth(self.index);
            self.index += 1;
            c
        } else {
            None
        }
    }
}

// tests for input reader

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Read, Stdout};

    use crate::file_ops::{get_file, get_slk_section};

    use super::*;

    #[test]
    fn test_input_reader() {
        let template = &get_file("tests/test-files/test-template.html");
        let content = get_slk_section(template, "slk-template");
        let input = content.to_string();
        let mut reader = InputReader::new(input);
        for _ in 0..reader.input.len()-1 {
            print!("{:?}", reader.next_char().unwrap());
        }
        assert_ne!(reader.input, "");
    }
}

