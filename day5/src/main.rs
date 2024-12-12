#![allow(unused_variables, unused_mut)]
use file_reader::read_text_file;

fn main() {
    let contents = read_text_file();
    let rules: Vec<(i32, i32)> = contents
        .split_whitespace()
        .filter(|s| s.contains('|'))
        .map(|s| {
            let parts: Vec<&str> = s.split('|').collect();
            let parts: Vec<&str> = s.split('|').collect();
            (
                parts[0].parse().expect("Failed to parse first number"),
                parts[1].parse().expect("Failed to parse second number"),
            )
        })
        .collect();

    let lists: Vec<Vec<i32>> = contents
        .split_whitespace()
        .filter(|s| s.contains(','))
        .map(|s| {
            s.split(',')
                .map(|num| num.parse().expect("Failed to parse number in list"))
                .collect()
        })
        .collect();

    let mut total = 0;

    for list in &lists {
        if list_is_valid(list, &rules) {
            total += list[list.len() / 2];
        }
    }

    println!("TOTAL: {}", total);
}

fn list_is_valid(list: &[i32], rules: &[(i32, i32)]) -> bool {
    for (i, &num) in list.iter().enumerate() {
        let new_rules: Vec<(i32, i32)> = rules
            .iter()
            .filter(|&&rule| rule.0 == num)
            .cloned()
            .collect();

        for &j in &list[0..i] {
            if new_rules.iter().any(|&rule| rule == (num, j)) {
                return false;
            }
        }
    }
    true
}
