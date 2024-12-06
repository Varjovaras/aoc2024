use file_reader::read_text_file;

fn main() {
    let contents = read_text_file();
    let reports: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|part| part.parse().unwrap())
                .collect()
        })
        .collect();

    let mut total_safe_reports = 0;
    for report in reports {
        if is_report_safe(&report) || is_report_fixable(&report) {
            total_safe_reports += 1;
        }
    }

    println!("{}", total_safe_reports);
}

fn is_report_safe(report: &[i32]) -> bool {
    let first_direction = report[0] - report[1] > 0;

    for (index, _) in report.iter().enumerate().take(report.len() - 1) {
        let distance = report[index] - report[index + 1];
        if distance == 0 {
            return false;
        }
        if distance.abs() > 3 || distance.abs() < 1 {
            return false;
        }
        if (report[index] - report[index + 1] > 0) != first_direction {
            return false;
        }
    }
    true
}

fn is_report_fixable(report: &[i32]) -> bool {
    for i in 0..report.len() {
        // Create a new vector without the element at index i
        let potentially_fixed_report: Vec<i32> = report
            .iter()
            .enumerate()
            .filter(|&(index, _)| index != i)
            .map(|(_, &val)| val)
            .collect();
        if is_report_safe(&potentially_fixed_report) {
            return true;
        }
    }
    false
}
