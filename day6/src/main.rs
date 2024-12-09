#![allow(unused_variables, unused_mut)]

use file_reader::read_text_file;

fn main() {
    let contents = read_text_file();
    let mut grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let arrow_coordinates: Option<(usize, usize)> = grid.iter().enumerate().find_map(|(i, row)| {
        row.iter().enumerate().find_map(
            |(j, char)| {
                if is_arrow(char) {
                    Some((i, j))
                } else {
                    None
                }
            },
        )
    });

    let (mut x, mut y) = arrow_coordinates.unwrap();

    loop {
        dbg!(x);
        dbg!(y);
        dbg!(grid[x][y]);
        if next_move_is_out_of_bounds(x, y, &grid) {
            grid[x][y] = 'X';
            break;
        }
        if move_arrow(x, y, &grid) {
            match grid[x][y] {
                '^' => {
                    grid[x - 1][y] = '^';
                    grid[x][y] = 'X';
                    x -= 1;
                }
                '>' => {
                    grid[x][y + 1] = '>';
                    grid[x][y] = 'X';
                    y += 1;
                }
                'v' => {
                    grid[x + 1][y] = 'v';
                    grid[x][y] = 'X';
                    x += 1;
                }
                '<' => {
                    grid[x][y - 1] = '<';
                    grid[x][y] = 'X';
                    y -= 1;
                }
                _ => {
                    panic!("should never happen")
                }
            }
        } else {
            grid[x][y] = turn_arrow(grid[x][y])
        }
    }
    println!("{:?}", count_number_of_visited_squares(&grid));
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
    match grid[x][y] {
        '^' => x == 0,
        '>' => y == grid[0].len() - 1,
        'v' => x == grid.len() - 1,
        '<' => y == 0,
        _ => panic!("wrong coordinates for the character"),
    }
}

fn is_arrow(char: &char) -> bool {
    char == &'^' || char == &'v' || char == &'<' || char == &'>'
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

fn count_number_of_visited_squares(grid: &[Vec<char>]) -> i32 {
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
