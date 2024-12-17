#![allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
use file_reader::read_text_file;

fn main() {
    let contents = read_text_file();
    let input_as_vector_of_i64: Vec<i64> = contents
        .split("")
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let mut converted_input: Vec<Option<i64>> = vec![];

    let mut number_to_add = 0;

    //convert input into Nones and i64 according to day9 specification
    for (i, number) in input_as_vector_of_i64.iter().enumerate() {
        if i % 2 != 0 {
            for _ in 0..*number {
                converted_input.push(None);
            }
        } else {
            for _ in 0..*number {
                converted_input.push(Some(number_to_add));
            }
            number_to_add += 1;
        }
    }

    while !has_continuous_some(&converted_input) {
        let first_none_index = converted_input
            .iter()
            .position(std::option::Option::is_none)
            .expect("No Nones found");

        let last_some_index = converted_input
            .iter()
            .rposition(std::option::Option::is_some)
            .expect("No some(i32) found");

        converted_input.swap(first_none_index, last_some_index);
    }

    let mut total = 0;

    //filter out nones before iterating
    for (i, num) in converted_input
        .iter()
        .filter_map(|&x| x)
        .collect::<Vec<i64>>()
        .iter()
        .enumerate()
    {
        total += i as i64 * *num;
    }

    println!("{total}");
}

fn has_continuous_some(ali: &[Option<i64>]) -> bool {
    let first_some_index = ali.iter().position(std::option::Option::is_some);
    let last_some_index = ali.iter().rposition(std::option::Option::is_some);

    match (first_some_index, last_some_index) {
        (Some(first), Some(last)) => ali[first..=last].iter().all(std::option::Option::is_some),
        _ => false,
    }
}
