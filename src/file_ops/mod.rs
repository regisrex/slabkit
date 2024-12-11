use std::{
    fs::File,
    io::{BufReader, Read},
    string,
};

use crate::logger::Logger;
use regex::Regex;
use serde_json::Value;

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

/*
   @param html: &str
   @param json: &Value
   @return String

   takes in the html string and json data and returns the html string with all the placeholders replaced with the json data.

*/


fn replace_json_in_html(html: &str, json: &Value) -> String {
    println!("===================");
    println!("HTML: {:?}", html);
    println!("JSON: {:?}", json);
    println!("===================");

    let re = Regex::new(r"\{\{(.*?)\}\}").unwrap(); // Matches {{key}} placeholders

    re.replace_all(html, |caps: &regex::Captures| {
        let key = &caps[1].trim();
        println!("Key: {:?}", key);

        // Check if key is nested by checking for dot notation (e.g., person.name)
        let parts: Vec<&str> = key.split('.').collect();
        let mut value = json;

        // Traverse nested JSON keys
        for part in parts {
            if let Some(v) = value.get(part) {
                value = v;
            } else {
                println!("Key not found: {}", key);
                return "".to_string();
            }
        }

        // Return the final value as a string, removing quotes if present
        let result = value.to_string();
        result.trim_matches('"').to_string()
    })
    .into_owned()
}
/*
   @param template: &String
   @param data: &Option<String>
   @return String

   takes in the template and data and returns the compiled html string with all html values replaced with the json data.

*/

pub fn compile_slk_contents(template: &String, data: &Option<String>) -> String {
    let slk_section: String = get_slk_section(template, "slk-section");
    let slk_json_data = match data {
        Some(data) => data,
        None => &get_slk_section(template, "slk-previewdata"),
    };

    let json_data: Value = serde_json::from_str(slk_json_data).unwrap();
    let compiled = replace_json_in_html(&slk_section, &json_data);
    compiled
}

/*
@param template: &String
@param section: &str
@return String

takes in the template string and section of the template to extract and returns the section of the template.
*/
// pub fn get_slk_section(template: &str, section: &str) -> String {
//     // Construct the regex pattern dynamically
//     print!("Template: {:?}", template);
//     let matching_string = format!(r"<{section}>(.*?)</{section}>", section = section);
//     println!("Matching string: {:?}", matching_string);

//     let re = Regex::new(&matching_string).unwrap();

//     // Search for the first capture
//     if let Some(cap) = re.captures(template) {
//         if let Some(matched_section) = cap.get(1) {
//             return matched_section.as_str().to_string();
//         }
//     }

//     println!("No matching section found for <{}>", section);
//     String::new()
// }


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