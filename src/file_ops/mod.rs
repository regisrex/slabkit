use crate::{logger::Logger, parser::Parser};
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



pub fn get_slk_section(template: &str, section: &str) -> Result<String, String> {
    // Construct regex dynamically with dot-all flag to handle multiline
    let matching_string = format!(r"(?s)<{section}>(.*?)</{section}>", section = section);
    println!("Matching string: {:?}", matching_string);

    let re = Regex::new(&matching_string).unwrap();

    // Check for captures and return matched content
    if let Some(cap) = re.captures(template) {
        if let Some(matched_section) = cap.get(1) {
            return Ok(matched_section.as_str().to_string())
        }
    }

    println!("No matching section found for <{}>", section);
    Err(format!("Slabkit error: couldn't resolve section {} from the provided template", template))
}

pub fn get_json_value_from_template(json: String) -> Value {
    let json_value = serde_json::from_str(&json);
    match json_value {
        Ok(value) => value,
        Err(_) => {
            println!("Slabkit error: Couldn't parse json data");
            Value::Null
        }
    }
}