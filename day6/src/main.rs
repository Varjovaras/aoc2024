#![allow(unused_variables, unused_mut)]

use file_reader::read_text_file;

fn main() {
    let contents = read_text_file();
    let mut rows: Vec<Vec<char>> = contents
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

    let (mut x, mut y) = start_sq.unwrap();

    loop {
        move_arrow(x, y, &rows);
        if next_move_is_out_of_bounds(x, y, &rows) {
            break;
        }
    }
}

fn next_move_is_out_of_bounds(x: usize, y: usize, rows: &[Vec<char>]) -> bool {
    todo!()
}

fn move_arrow(x: usize, y: usize, rows: &[Vec<char>]) {
    let char = rows[x][y];
    match char {
        '^' => try_to_move_up(),
        '>' => try_to_move_right(),
        'v' => try_to_move_down(),
        '<' => try_to_move_left(),
        _ => panic!("how??"),
    }
}

fn try_to_move_left() {
    todo!()
}

fn try_to_move_right() {
    todo!()
}

fn try_to_move_down() {
    todo!()
}

fn try_to_move_up() {
    todo!()
}

fn is_guard(char: &char) -> bool {
    char == &'^' || char == &'v' || char == &'<' || char == &'>'
}
