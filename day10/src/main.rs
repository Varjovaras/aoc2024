use file_reader::read_text_file;

fn main() {
    let contents = read_text_file();
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    println!("Hello, world!");
    print_grid(&grid);
}

fn print_grid(input: &[Vec<char>]) {
    let mut strings: Vec<String> = Vec::with_capacity(input.len());
    (0..input.len()).for_each(|i| {
        let mut result_string = String::with_capacity(input[i].len());
        for j in 0..input[i].len() {
            result_string.push(input[i][j]);
        }
        strings.push(result_string);
    });
    for string in strings {
        println!("{string}");
    }
}
