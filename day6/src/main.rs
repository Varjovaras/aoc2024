#![allow(unused_variables, unused_mut)]

use file_reader::read_text_file;

fn main() {
    let contents = read_text_file();
    let mut grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (mut x, mut y) = find_coordinates(&grid).expect("coordinates not found");
    let mut total = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let original_char = grid[i][j];
            if !is_arrow(original_char) {
                grid[i][j] = '#';
            };
            if !game_completes(&grid) {
                total += 1;
            }
            grid[i][j] = original_char;
        }
    }
    println!("{total}");
}

fn game_completes(grid: &[Vec<char>]) -> bool {
    let (mut x, mut y) = find_coordinates(grid).expect("coordinates not found");

    let mut new_grid = grid.to_owned();

    //100_000 iterations should be way more than enough
    for i in 0..100_000 {
        if next_move_is_out_of_bounds(x, y, &new_grid) {
            new_grid[x][y] = 'X';
            return true;
        }
        if move_arrow(x, y, &new_grid) {
            match new_grid[x].get(y) {
                Some('^') => {
                    new_grid[x - 1][y] = '^';
                    new_grid[x][y] = 'X';
                    x -= 1;
                }
                Some('>') => {
                    new_grid[x][y + 1] = '>';
                    new_grid[x][y] = 'X';
                    y += 1;
                }
                Some('v') => {
                    new_grid[x + 1][y] = 'v';
                    new_grid[x][y] = 'X';
                    x += 1;
                }
                Some('<') => {
                    new_grid[x][y - 1] = '<';
                    new_grid[x][y] = 'X';
                    y -= 1;
                }
                _ => {
                    panic!("should never happen")
                }
            }
        } else {
            new_grid[x][y] = turn_arrow(new_grid[x][y]);
        }
    }
    false
}

fn move_arrow(x: usize, y: usize, grid: &[Vec<char>]) -> bool {
    let char = grid[x][y];
    match char {
        '^' => square_is_available(grid[x - 1][y]),

        '>' => square_is_available(grid[x][y + 1]),

        'v' => square_is_available(grid[x + 1][y]),

        '<' => square_is_available(grid[x][y - 1]),
        _ => panic!("how??"),
    }
}

fn turn_arrow(char: char) -> char {
    match char {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => {
            panic!("Should not happen!!")
        }
    }
}

fn next_move_is_out_of_bounds(x: usize, y: usize, grid: &[Vec<char>]) -> bool {
    match grid[x].get(y) {
        Some('^') => x == 0,
        Some('>') => y == grid[0].len() - 1,
        Some('v') => x == grid.len() - 1,
        Some('<') => y == 0,
        _ => panic!("Wrong coordinates for the character"),
    }
}

const fn is_arrow(char: char) -> bool {
    char == '^' || char == 'v' || char == '<' || char == '>'
}

fn square_is_available(char: char) -> bool {
    match char {
        '.' | 'X' => true,
        '#' => false,
        _ => {
            panic!("there shouldn't be other than . and # characters")
        }
    }
}

fn find_coordinates(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    grid.iter().enumerate().find_map(|(i, row)| {
        row.iter().enumerate().find_map(
            |(j, char)| {
                if is_arrow(*char) {
                    Some((i, j))
                } else {
                    None
                }
            },
        )
    })
}

//Solution 1
fn _count_number_of_visited_squares(grid: &[Vec<char>]) -> i32 {
    let mut total = 0;
    for row in grid {
        for char in row {
            if char == &'X' {
                total += 1;
            }
        }
    }
    total
}
