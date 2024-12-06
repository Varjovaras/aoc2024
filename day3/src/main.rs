use file_reader::read_text_file;
use std::char;

fn main() {
    let contents = read_text_file();
    let chars: Vec<char> = contents.chars().collect();
    let mut total = 0;
    let mut allow_multiplying = true;

    for (index, char) in chars.iter().enumerate() {
        if char == &'d'
            && chars.get(index + 1) == Some(&'o')
            && chars.get(index + 2) == Some(&'(')
            && chars.get(index + 3) == Some(&')')
        {
            allow_multiplying = true;
        }
        if char == &'d'
            && chars.get(index + 1) == Some(&'o')
            && chars.get(index + 2) == Some(&'n')
            && chars.get(index + 3) == Some(&'\'')
            && chars.get(index + 4) == Some(&'t')
            && chars.get(index + 5) == Some(&'(')
            && chars.get(index + 6) == Some(&')')
        {
            allow_multiplying = false
        }
        if allow_multiplying
            && char == &'m'
            && chars.get(index + 1) == Some(&'u')
            && chars.get(index + 2) == Some(&'l')
            && chars.get(index + 3) == Some(&'(')
        {
            let mut first_number_as_string = String::new();
            let mut second_number_as_string = String::new();
            let mut check_second_number = false;
            let mut current_index = index + 4;

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
                        let mul = first_number_as_string.parse::<i32>().unwrap()
                            * second_number_as_string.parse::<i32>().unwrap();
                        total += mul;
                        break;
                    }
                    _ => break,
                }
                current_index += 1;
            }
        }
    }
    println!("{}", total);
}
