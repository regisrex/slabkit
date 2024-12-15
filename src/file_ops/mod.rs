use crate::{lexer::Lexer, logger::Logger, parser::Parser};
use regex::Regex;
use serde_json::Value;
use std::{
    fs::File,
    io::{BufReader, Read},
};

/*
   @param path: &str
   @return String

   takes in the path to the file and returns the contents of the file as a string.

*/
pub fn get_file(path: &str) -> String {
    let logger = Logger::new("INFO".to_string(), "Getting file".to_string());
    let file = File::open(path);
    match file {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut contents = String::new();
            let reader_result = reader.read_to_string(&mut contents);
            match reader_result {
                Ok(_) => contents,
                Err(_) => {
                    logger.error("Error reading file");
                    String::from("")
                }
            }
        }
        Err(_) => {
            logger.error("File not found");
            String::from("")
        }
    }
}

fn replace_json_in_html_recursively(html: &str, json: &Value) -> String {
    println!("HTML: {:?}", html);
    let mut parser = Parser::new(html.to_string());
    for token in parser.tokens.iter() {
        println!("{:?}", token);
    }


    println!("============ PARSING ============");
    let node = parser.parse();

    // println!("Node after parsing: {:?}", node);

    "".to_string()
}

pub fn compile_slk_contents(template: &String, data: &Option<String>) -> String {
    let slk_section: String = get_slk_section(template, "slk-section");
    let slk_json_data = match data {
        Some(data) => data,
        None => &get_slk_section(template, "slk-previewdata"),
    };

    let json_data: Value = serde_json::from_str(slk_json_data).unwrap();
    let compiled = replace_json_in_html_recursively(&slk_section, &json_data);
    compiled
}

pub fn get_slk_section(template: &str, section: &str) -> String {
    // Construct regex dynamically with dot-all flag to handle multiline
    let matching_string = format!(r"(?s)<{section}>(.*?)</{section}>", section = section);
    println!("Matching string: {:?}", matching_string);

    let re = Regex::new(&matching_string).unwrap();

    // Check for captures and return matched content
    if let Some(cap) = re.captures(template) {
        if let Some(matched_section) = cap.get(1) {
            return matched_section.as_str().to_string();
        }
    }

    println!("No matching section found for <{}>", section);
    String::new()
}
