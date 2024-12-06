use std::fs;

pub fn read_text_file() -> String {
    let file_path = &String::from("input.txt");
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}
