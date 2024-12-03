use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let vec: Vec<&str> = contents.split("\n").collect();
    let vec_tuples: Vec<(i32, i32)> = vec
        .iter()
        .filter_map(|s| {
            let parts: Vec<&str> = s.split_whitespace().collect();
            if parts.len() >= 2 {
                Some((parts[0].parse().ok()?, parts[1].parse().ok()?))
            } else {
                None
            }
        })
        .collect();
    let mut vec1: Vec<i32> = vec![];
    let mut vec2: Vec<i32> = vec![];

    for (first, second) in vec_tuples {
        vec1.push(first);
        vec2.push(second);
    }

    vec1.sort();
    vec2.sort();

    let vec_tuples: Vec<(i32, i32)> = vec1
        .iter()
        .zip(vec2.iter())
        .map(|(a, b)| (*a, *b))
        .collect();

    let mut total = 0;
    for tuple in vec_tuples {
        total += tuple.0.abs_diff(tuple.1);
    }

    dbg!(total);
}
