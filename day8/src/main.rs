use file_reader::read_text_file;

fn main() {
    let contents = read_text_file();
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut antinode_locations: Vec<(usize, usize)> = vec![];

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let locations = calculate_antinodes((i, j), &grid);
            antinode_locations.extend(locations);
        }
    }

    dbg!(antinode_locations.len());

    dbg!("total");
}

fn calculate_antinodes(ali: (usize, usize), grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let antinodes = vec![];
    let char = grid[ali.0][ali.1];
    if char == '.' {
        return antinodes;
    }

    find_matching_characters(char, grid)
}

fn find_matching_characters(char: char, grid: &[Vec<char>]) -> Vec<(usize, usize)> {
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
