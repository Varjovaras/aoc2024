use file_reader::read_text_file;

fn main() {
    let contents = read_text_file();
    let mut total = 0;
    let rows: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    for (i, row) in rows.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if is_x_shaped_mas(&rows, i, j) {
                total += 1
            }
        }
    }

    println!("{}", total);
    dbg!(total);
}

#[allow(clippy::nonminimal_bool)]
fn is_x_shaped_mas(rows: &[Vec<char>], i: usize, j: usize) -> bool {
    if rows[i][j] != 'A' || i == 0 || j == 0 || i + 1 >= rows.len() || j + 1 >= rows[0].len() {
        return false;
    }

    let top_left_to_bottom_right =
        is_mas_forwards_or_backwards(&rows[i - 1][j - 1], &rows[i + 1][j + 1]);
    let bottom_left_to_top_right =
        is_mas_forwards_or_backwards(&rows[i + 1][j - 1], &rows[i - 1][j + 1]);

    top_left_to_bottom_right && bottom_left_to_top_right
}

fn is_mas_forwards_or_backwards(first: &char, second: &char) -> bool {
    first == &'S' && second == &'M' || first == &'M' && second == &'S'
}
