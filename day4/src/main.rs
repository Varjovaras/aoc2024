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
            total += check_xmases(&rows, i, j);
        }
    }

    println!("{}", total);
    dbg!(total);
}

fn check_xmases(rows: &[Vec<char>], i: usize, j: usize) -> i32 {
    let checks = [
        is_xmas_right,
        is_xmas_left,
        is_xmas_down,
        is_xmas_up,
        is_xmas_up_right,
        is_xmas_up_left,
        is_xmas_down_left,
        is_xmas_down_right,
    ];

    checks.iter().filter(|&&check| check(rows, i, j)).count() as i32
}

fn is_xmas_left(rows: &[Vec<char>], i: usize, j: usize) -> bool {
    if j < 4 {
        return false;
    }

    rows[i][j - 4..j] == ['X', 'M', 'A', 'S']
}

fn is_xmas_right(rows: &[Vec<char>], i: usize, j: usize) -> bool {
    if j + 4 > rows[i].len() {
        return false;
    }

    rows[i][j..j + 4] == ['X', 'M', 'A', 'S']
}

fn is_xmas_down(rows: &[Vec<char>], i: usize, j: usize) -> bool {
    if i + 4 > rows.len() {
        return false;
    }

    rows[i][j] == 'X' && rows[i + 1][j] == 'M' && rows[i + 2][j] == 'A' && rows[i + 3][j] == 'S'
}

fn is_xmas_up(rows: &[Vec<char>], i: usize, j: usize) -> bool {
    if i < 3 {
        return false;
    }

    rows[i][j] == 'X' && rows[i - 1][j] == 'M' && rows[i - 2][j] == 'A' && rows[i - 3][j] == 'S'
}

fn is_xmas_up_right(rows: &[Vec<char>], i: usize, j: usize) -> bool {
    if i < 3 || j + 4 > rows[i].len() {
        return false;
    }
    rows[i][j] == 'X'
        && rows[i - 1][j + 1] == 'M'
        && rows[i - 2][j + 2] == 'A'
        && rows[i - 3][j + 3] == 'S'
}

fn is_xmas_up_left(rows: &[Vec<char>], i: usize, j: usize) -> bool {
    if i < 3 || j < 4 {
        return false;
    }
    rows[i][j] == 'X'
        && rows[i - 1][j - 1] == 'M'
        && rows[i - 2][j - 2] == 'A'
        && rows[i - 3][j - 3] == 'S'
}

fn is_xmas_down_left(rows: &[Vec<char>], i: usize, j: usize) -> bool {
    if i + 4 > rows.len() || j < 4 {
        return false;
    }
    rows[i][j] == 'X'
        && rows[i + 1][j - 1] == 'M'
        && rows[i + 2][j - 2] == 'A'
        && rows[i + 3][j - 3] == 'S'
}

fn is_xmas_down_right(rows: &[Vec<char>], i: usize, j: usize) -> bool {
    if i + 4 > rows.len() || j + 4 > rows[i].len() {
        return false;
    }
    rows[i][j] == 'X'
        && rows[i + 1][j + 1] == 'M'
        && rows[i + 2][j + 2] == 'A'
        && rows[i + 3][j + 3] == 'S'
}
