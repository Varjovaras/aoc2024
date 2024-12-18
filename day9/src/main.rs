#![allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]

use file_reader::read_text_file;

fn main() {
    let contents = read_text_file();
    let input_as_vector_of_i64: Vec<i64> = contents
        .split("")
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let converted_input: Vec<Option<i64>> = convert_input(&input_as_vector_of_i64);

    let solution1 = solution1(&converted_input);
    let solution2 = solution2(converted_input);

    let solution1_total = count_total(&solution1);
    let solution2_total = count_total(&solution2);

    println!("{solution1_total}");
    println!("{solution2_total}");
}

fn convert_input(input: &[i64]) -> Vec<Option<i64>> {
    let mut new_input: Vec<Option<i64>> = vec![];
    let mut number_to_add = 0;

    //convert input into Nones and i64 according to day9 assignment
    for (i, number) in input.iter().enumerate() {
        if i % 2 != 0 {
            for _ in 0..*number {
                new_input.push(None);
            }
        } else {
            for _ in 0..*number {
                new_input.push(Some(number_to_add));
            }
            number_to_add += 1;
        }
    }
    new_input
}

//solution 2
fn solution2(input: Vec<Option<i64>>) -> Vec<Option<i64>> {
    let mut new_input = input;

    let mut number_to_check: Option<i64> = None;

    for i in (0..new_input.len()).rev() {
        if new_input[i] == number_to_check || new_input[i].is_none() {
            continue;
        }

        let some_count = count_consecutive_from_index(&new_input, i);

        let mut none_count = 0;
        let mut none_index = 0;

        for j in 0..i {
            if new_input[j].is_none() {
                none_index = j;
                none_count = count_consecutive_none_from_index(&new_input, none_index);
            }

            if none_count >= some_count {
                let swap_count = some_count.min(none_count);

                for k in 0..swap_count {
                    new_input.swap(i - k, none_index + k);
                }
                break;
            }
            none_count = 0;
        }
        number_to_check = new_input[i];
    }
    new_input
}

fn count_consecutive_from_index(input: &[Option<i64>], start_index: usize) -> usize {
    let number =
        input[start_index].expect("Number should've been available because of .rposition(is_some)");

    let mut count = 0;

    for &item in input[..=start_index].iter().rev() {
        if item == Some(number) {
            count += 1;
        } else {
            break;
        }
    }

    count
}

fn count_consecutive_none_from_index(vec: &[Option<i64>], start_index: usize) -> usize {
    let mut count = 0;

    for &item in &vec[start_index..] {
        if item.is_none() {
            count += 1;
        } else {
            break;
        }
    }

    count
}

fn count_total(input: &[Option<i64>]) -> i64 {
    let mut total = 0;
    (0..input.len()).for_each(|i| {
        if let Some(al) = input[i] {
            total += al * i as i64;
        }
    });
    total
}

fn _print_input(input: &[Option<i64>]) {
    let mut result_string = String::new();
    (0..input.len()).for_each(|i| {
        if let Some(al) = input[i] {
            if al > 9 {
                dbg!(al);
                panic!(":D");
            }
            let first_digit = al.to_string().chars().next().expect("");
            if let Some(c) = char::from_digit(first_digit.to_digit(10).expect("??"), 10) {
                result_string.push(c);
            }
        } else {
            result_string.push('.');
        }
    });
    dbg!(result_string);
}

fn solution1(input: &[Option<i64>]) -> Vec<Option<i64>> {
    let mut new_input = input.to_vec();
    while !has_continuous_some(&new_input) {
        let first_none_index = new_input
            .iter()
            .position(std::option::Option::is_none)
            .expect("No Nones found");

        let last_some_index = new_input
            .iter()
            .rposition(std::option::Option::is_some)
            .expect("No some(i32) found");

        new_input.swap(first_none_index, last_some_index);
    }
    new_input
}

fn has_continuous_some(input: &[Option<i64>]) -> bool {
    let first_some_index = input.iter().position(std::option::Option::is_some);
    let last_some_index = input.iter().rposition(std::option::Option::is_some);

    match (first_some_index, last_some_index) {
        (Some(first), Some(last)) => input[first..=last].iter().all(std::option::Option::is_some),
        _ => false,
    }
}
