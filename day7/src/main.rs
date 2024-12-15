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
        let values: Result<Vec<i64>, _> = parts[1].split_whitespace().map(|s| s.parse()).collect();
        if let Ok(v) = values {
            input_data.push(InputData { key, values: v })
        }
    }

    input_data
}

fn line_is_true(line: &InputData) -> bool {
    let operator_combinations: Vec<String> = generate_operator_strings(line.values.len());
    let mut comb_strings: Vec<Vec<String>> = vec![];
    for operator_combination in operator_combinations {
        let mut str: Vec<String> = vec![];
        for i in 0..line.values.len() {
            str.push(line.values[i].to_string());
            if i < operator_combination.len() {
                str.push(operator_combination.chars().nth(i).unwrap().into());
            }
        }

        comb_strings.push(str);
    }

    for comb_string in comb_strings {
        if line.key == get_total(comb_string).unwrap() {
            return true;
        }
    }

    false
}

fn get_total(vec: Vec<String>) -> Result<i64, String> {
    let mut total = vec.first().unwrap().parse::<i64>().ok().unwrap();
    let mut i = 0;
    while i + 2 < vec.len() {
        match vec[i + 1].as_ref() {
            "+" => total += vec[i + 2].parse::<i64>().unwrap(),
            "*" => total *= vec[i + 2].parse::<i64>().unwrap(),
            "|" => {
                let left_side = total.to_string();
                let right_side = &vec[i + 2];
                let combined_numbers = left_side + right_side;
                total = combined_numbers.parse::<i64>().unwrap();
            }

            _ => return Err("Incorrect operator".to_string()),
        }
        i += 2;
    }
    Ok(total)
}

fn generate_operator_strings(num_numbers: usize) -> Vec<String> {
    let mut combinations = Vec::new();
    let operators = vec!['+', '*', '|'];

    let num_operators = num_numbers - 1;

    let mut current_combination = Vec::new();
    generate_combinations(
        &operators,
        &mut current_combination,
        num_operators,
        &mut combinations,
    );

    combinations
}

fn generate_combinations(
    operators: &[char],
    current_combination: &mut Vec<char>,
    num_operators: usize,
    combinations: &mut Vec<String>,
) {
    if current_combination.len() == num_operators {
        combinations.push(current_combination.iter().collect());
        return;
    }

    for &op in operators {
        current_combination.push(op);
        generate_combinations(operators, current_combination, num_operators, combinations);
        current_combination.pop();
    }
}
