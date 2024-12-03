use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let report_vectors: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|part| part.parse().unwrap())
                .collect()
        })
        .collect();

    let mut total_safe_reports = 0;
    for report in report_vectors {
        let mut safe = true;
        let first_direction = check_dir(report[0] - report[1]);

        for (index, _) in report.iter().enumerate().take(report.len() - 1) {
            let distance = report[index] - report[index + 1];
            if distance == 0 {
                safe = false;
                break;
            }
            if distance.abs() > 3 || distance.abs() < 1 {
                safe = false;
                break;
            }
            if check_dir(report[index] - report[index + 1]) != first_direction {
                safe = false;
                break;
            }
        }
        if safe {
            total_safe_reports += 1;
        }
    }

    println!("{}", total_safe_reports)
}

fn check_dir(i: i32) -> bool {
    i > 0
}
