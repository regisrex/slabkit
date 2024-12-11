pub struct Logger {
    pub level: String,
    pub output_file : String
}

impl Logger {
    pub fn new(level: String, output_file: String) -> Logger {
        Logger {
            level,
            output_file
        }
    }

    pub fn log(&self,message : &str) {
        println!("{}: {}", self.level, message);
    }

    pub fn error(&self, message: &str) {
        eprintln!("{}: {}", self.level, message);
    }

    pub fn warn(&self, message: &str) {
        println!("{}: {}", self.level, message);
    }
    


}
