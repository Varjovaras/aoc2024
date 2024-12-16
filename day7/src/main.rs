use file_reader::read_text_file;

#[derive(Debug)]
struct InputData {
    key: i64,
    values: Vec<i64>,
}

fn main() {
    let contents = parse_input_file();
    let mut total = 0;
    for line in contents {
        if line_is_true(&line) {
            total += line.key;
        }
    }
    dbg!(total);
}

fn parse_input_file() -> Vec<InputData> {
    let contents = read_text_file();
    let mut input_data: Vec<InputData> = vec![];
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let key = parts[0]
            .trim()
            .parse::<i64>()
            .expect("integer expected as key");
        let values: Result<Vec<i64>, _> = parts[1].split_whitespace().map(str::parse).collect();
        if let Ok(v) = values {
            input_data.push(InputData { key, values: v });
        }
    }
    input_data
}

fn line_is_true(line: &InputData) -> bool {
    let mut current_combination = Vec::new();
    generate_and_check(line, &mut current_combination, 0, 0)
}

//recursively check possible combinations and early return if any match total
fn generate_and_check(
    line: &InputData,
    current_combination: &mut Vec<char>,
    current_total: i64,
    depth: usize,
) -> bool {
    if depth == line.values.len() {
        return current_total == line.key;
    }

    let new_total = if depth == 0 {
        line.values[0]
    } else {
        match current_combination.get(depth - 1) {
            Some('+') => current_total + line.values[depth],
            Some('*') => current_total * line.values[depth],
            Some('|') => format!("{}{}", current_total, line.values[depth])
                .parse::<i64>()
                .expect(""),
            _ => unreachable!(),
        }
    };

    if new_total > line.key {
        return false;
    }

    let operators = ['+', '*', '|'];

    if depth < line.values.len() - 1 {
        for &op in &operators {
            current_combination.push(op);
            if generate_and_check(line, current_combination, new_total, depth + 1) {
                return true;
            }
            current_combination.pop();
        }
        false
    } else {
        new_total == line.key
    }
}
