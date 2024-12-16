
use crate::{file_ops, logger::Logger};

pub fn get_action( action : &Option<String> ) -> String {
    let logger = Logger::new(
        "INFO".to_string(),
        "Getting action".to_string(),
    );
    match action {
        Some(action) => {
            if action == "dev" || action == "compile" {
                action.to_string()
            } else {
                logger.error("Invalid action");
                String::from("dev")
            }
        },
        None => String::from("dev")
    }

}

pub fn get_template( template : &Option<String> ) -> String {
    let logger = Logger::new(
        "INFO".to_string(),
        "Getting template".to_string(),
    );
    match template {
        Some(template) => {
            file_ops::get_file(template)
        },
        None => {
            logger.error("No template provided");
            String::from("template.html")
        }
    }
}


pub fn get_data( template : &Option<String> ) -> Option<String> {
     match template {
        Some(template) => {
             Some(file_ops::get_file(template))
        },
        None => None
    }
}

pub fn get_outfile( output : &Option<String> ) -> String {
    let logger = Logger::new(
        "INFO".to_string(),
        "Getting output file".to_string(),
    );
    match output {
        Some(output) => {
           file_ops::get_file(output)
        },
        None => {
            logger.error("No output file provided");
            String::from("output.html")
        }
    }
}