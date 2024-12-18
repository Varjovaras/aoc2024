use file_reader::read_text_file;

fn main() {
    let contents = read_text_file();
    let vec: Vec<&str> = contents.lines().collect();
    let vec_tuples: Vec<(i32, i32)> = vec
        .iter()
        .filter_map(|s| {
            let parts: Vec<&str> = s.split_whitespace().collect();
            if parts.len() >= 2 {
                Some((parts[0].parse().ok()?, parts[1].parse().ok()?))
            } else {
                None
            }
        })
        .collect();
    let mut vec1: Vec<i32> = vec![];
    let mut vec2: Vec<i32> = vec![];

    for (first, second) in vec_tuples {
        vec1.push(first);
        vec2.push(second);
    }

    let mut similarity_score = 0;

    for i in &vec1 {
        let mut occurences = 0;
        for j in &vec2 {
            if j == i {
                occurences += 1;
            }
        }
        similarity_score += i * occurences;
    }

    println!("{similarity_score}");
}
