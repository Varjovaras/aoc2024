#![allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
use std::collections::HashSet;

use file_reader::read_text_file;

fn main() {
    let contents = read_text_file();
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut antinode_locations: HashSet<(usize, usize)> = HashSet::new();
    let mut radars_checked: HashSet<char> = HashSet::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let locations = calculate_antinodes((i, j), &grid, &radars_checked);
            for location in locations {
                antinode_locations.insert(location);
            }
            radars_checked.insert(grid[i][j]);
        }
    }

    dbg!(antinode_locations.len());
}

fn calculate_antinodes(
    start_position: (usize, usize),
    grid: &[Vec<char>],
    radars_checked: &HashSet<char>,
) -> Vec<(usize, usize)> {
    let char = grid[start_position.0][start_position.1];
    if char == '.' || char == '#' || radars_checked.contains(&char) {
        return vec![];
    }

    let matching_characters = find_matching_characters(grid, char);
    location_of_antinodes(&matching_characters, grid)
}

fn find_matching_characters(grid: &[Vec<char>], char: char) -> Vec<(usize, usize)> {
    let mut locations = vec![];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == char {
                locations.push((i, j));
            }
        }
    }
    locations
}

fn location_of_antinodes(locations: &[(usize, usize)], grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut antinode_locations = vec![];
    let x_len = grid.len() as isize;
    let y_len = grid[0].len() as isize;
    for i in 0..locations.len() {
        for j in i + 1..locations.len() {
            let first_location = (locations[i].0 as isize, locations[i].1 as isize);
            let second_location = (locations[j].0 as isize, locations[j].1 as isize);

            let diff_x = first_location.0 - second_location.0;
            let diff_y = first_location.1 - second_location.1;

            if first_location.0 + diff_x >= 0
                && first_location.0 + diff_x < x_len
                && first_location.1 + diff_y >= 0
                && first_location.1 + diff_y < y_len
            {
                antinode_locations.push((
                    (first_location.0 + diff_x) as usize,
                    (first_location.1 + diff_y) as usize,
                ));
            }

            if second_location.0 - diff_x >= 0
                && second_location.0 - diff_x < x_len
                && second_location.1 - diff_y >= 0
                && second_location.1 - diff_y < y_len
            {
                antinode_locations.push((
                    (second_location.0 - diff_x) as usize,
                    (second_location.1 - diff_y) as usize,
                ));
            }
        }
    }

    antinode_locations.sort_unstable();
    antinode_locations.dedup();

    antinode_locations
}
