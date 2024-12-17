use file_reader::read_text_file;

fn main() {
    let contents = read_text_file();
    let numbers: Vec<i32> = contents
        .split("")
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    let mut new_characters: Vec<Option<i32>> = vec![];

    let mut number_to_add = 0;

    for (i, number) in numbers.iter().enumerate() {
        if i % 2 != 0 {
            for _ in 0..*number {
                new_characters.push(None);
            }
        } else {
            for _ in 0..*number {
                new_characters.push(Some(number_to_add));
            }
            number_to_add += 1;
        }
    }

    // dbg!(new_characters);
}
