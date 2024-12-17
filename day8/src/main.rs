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
    let current_character = grid[start_position.0][start_position.1];
    if current_character == '.'
        || current_character == '#'
        || radars_checked.contains(&current_character)
    {
        return vec![];
    }

    let matching_characters = find_matching_characters(grid, current_character);
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
            let first_antenna = (locations[i].0 as isize, locations[i].1 as isize);
            let second_antenna = (locations[j].0 as isize, locations[j].1 as isize);

            let diff_x = second_antenna.0 - first_antenna.0;
            let diff_y = second_antenna.1 - first_antenna.1;

            let mut current_x = first_antenna.0;
            let mut current_y = first_antenna.1;

            while current_x >= 0 && current_x < x_len && current_y >= 0 && current_y < y_len {
                antinode_locations.push((current_x as usize, current_y as usize));

                current_x += diff_x;
                current_y += diff_y;
            }

            current_x = second_antenna.0;
            current_y = second_antenna.1;

            while current_x >= 0 && current_x < x_len && current_y >= 0 && current_y < y_len {
                antinode_locations.push((current_x as usize, current_y as usize));

                current_x -= diff_x;
                current_y -= diff_y;
            }
        }
    }

    antinode_locations.sort_unstable();
    antinode_locations.dedup();
    antinode_locations
}
