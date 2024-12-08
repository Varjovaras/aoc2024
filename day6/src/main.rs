use file_reader::read_text_file;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let contents = read_text_file();
    let rows: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let start_sq: Option<(usize, usize)> = rows.iter().enumerate().find_map(|(i, row)| {
        row.iter().enumerate().find_map(
            |(j, char)| {
                if is_guard(char) {
                    Some((i, j))
                } else {
                    None
                }
            },
        )
    });

    println!("Hello, world!");
}

fn is_guard(char: &char) -> bool {
    char == &'^' || char == &'v' || char == &'<' || char == &'>'
}
