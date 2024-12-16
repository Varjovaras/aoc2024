use file_reader::read_text_file;
use std::char;

fn main() {
    let contents = read_text_file();
    let chars: Vec<char> = contents.chars().collect();
    let mut total = 0;
    let mut allow_multiplying = true;

    for (index, char) in chars.iter().enumerate() {
        match *char {
            'd' if is_do_command(&chars, index) => allow_multiplying = true,

            'd' if is_dont_command(&chars, index) => {
                allow_multiplying = false;
            }
            'm' if allow_multiplying && is_mul_command(&chars, index) => {
                total += increment_total(index, &chars);
            }
            _ => {}
        }
    }
    println!("{total}");
}

fn is_do_command(chars: &[char], index: usize) -> bool {
    index + 3 < chars.len() && chars[index..=index + 3] == ['d', 'o', '(', ')']
}

fn is_dont_command(chars: &[char], index: usize) -> bool {
    index + 6 < chars.len() && chars[index..=index + 6] == ['d', 'o', 'n', '\'', 't', '(', ')']
}

fn is_mul_command(chars: &[char], index: usize) -> bool {
    index + 2 < chars.len() && chars[index..=index + 2] == ['m', 'u', 'l']
}

fn increment_total(index: usize, chars: &[char]) -> i32 {
    let mut first_number_as_string = String::new();
    let mut second_number_as_string = String::new();
    let mut check_second_number = false;
    let mut current_index = index + 4;
    let mut total = 0;

    while let Some(next_char) = chars.get(current_index) {
        match next_char {
            c if c.is_ascii_digit() && !check_second_number => {
                first_number_as_string.push(*c);
            }
            c if c.is_ascii_digit() && check_second_number => {
                second_number_as_string.push(*c);
            }
            &',' if !first_number_as_string.is_empty() => {
                check_second_number = true;
            }
            &')' if check_second_number && !second_number_as_string.is_empty() => {
                let mul = first_number_as_string
                    .parse::<i32>()
                    .expect("first number didnt parse into i32")
                    * second_number_as_string
                        .parse::<i32>()
                        .expect("second number didnt parse into i32");
                total += mul;
                break;
            }
            _ => break,
        }
        current_index += 1;
    }
    total
}
