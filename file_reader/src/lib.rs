use std::{env, fs};

pub fn read_text_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}
