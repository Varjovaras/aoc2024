use file_reader::read_text_file;

fn main() {
    let contents = read_text_file();
    let mut total = 0;
    let horizontal_rows: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let vertical_rows: Vec<Vec<char>> = (0..horizontal_rows[0].len())
        .map(|col_index| horizontal_rows.iter().map(|row| row[col_index]).collect())
        .collect();

    for line in &horizontal_rows {
        total += check_row(line);
    }
    for line in &vertical_rows {
        total += check_row(line)
    }

    total += get_diagonal_rows(&horizontal_rows);

    println!("{}", total);
    dbg!(total);
}

fn is_xmas(line: &[char], index: usize) -> bool {
    index + 3 < line.len() && line[index..=index + 3] == ['X', 'M', 'A', 'S']
}

fn is_xmas_backwards(line: &[char], index: usize) -> bool {
    index + 3 < line.len() && line[index..=index + 3] == ['S', 'A', 'M', 'X']
}

fn check_row(row: &[char]) -> i32 {
    let mut total = 0;
    for (index, _) in row.iter().enumerate() {
        if is_xmas(row, index) || is_xmas_backwards(row, index) {
            total += 1;
        }
    }
    total
}
fn get_diagonal_rows(horizontal_rows: &[Vec<char>]) -> i32 {
    let mut diagonals = Vec::new();
    let mut total = 0;

    // Diagonals starting from first column
    for start_row in 0..horizontal_rows.len() {
        let diagonal: Vec<char> = (0..)
            .zip(start_row..)
            .take_while(|&(col, row)| {
                row < horizontal_rows.len() && col < horizontal_rows[row].len()
            })
            .filter_map(|(col, row)| horizontal_rows[row].get(col).copied())
            .collect();

        if !diagonal.is_empty() {
            diagonals.push(diagonal);
        }
    }

    // Diagonals starting from first row
    for start_col in 1..horizontal_rows[0].len() {
        let diagonal: Vec<char> = (start_col..)
            .zip(0..)
            .take_while(|&(col, row)| {
                row < horizontal_rows.len() && col < horizontal_rows[row].len()
            })
            .filter_map(|(col, row)| horizontal_rows[row].get(col).copied())
            .collect();

        if !diagonal.is_empty() {
            diagonals.push(diagonal);
        }
    }

    // Bottom-left to top-right diagonals starting from first column
    for start_row in (0..horizontal_rows.len()).rev() {
        let diagonal: Vec<char> = (0..)
            .zip((0..=start_row).rev())
            .take_while(|&(col, row)| col < horizontal_rows[row].len())
            .filter_map(|(col, row)| horizontal_rows[row].get(col).copied())
            .collect();

        if !diagonal.is_empty() {
            diagonals.push(diagonal);
        }
    }

    // Bottom-left to top-right diagonals starting from first row
    for start_col in 1..horizontal_rows[0].len() {
        let diagonal: Vec<char> = (start_col..)
            .zip((0..horizontal_rows.len()).rev())
            .take_while(|&(col, row)| col < horizontal_rows[row].len())
            .filter_map(|(col, row)| horizontal_rows[row].get(col).copied())
            .collect();

        if !diagonal.is_empty() {
            diagonals.push(diagonal);
        }
    }

    for row in &diagonals {
        total += check_row(row);
    }

    total
}
