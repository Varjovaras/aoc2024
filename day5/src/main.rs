use file_reader::read_text_file;

fn main() {
    let contents = read_text_file();
    let rules: Vec<(i32, i32)> = contents
        .split_whitespace()
        .filter(|s| s.contains('|'))
        .map(|s| {
            let parts: Vec<&str> = s.split('|').collect();
            (
                parts[0].parse().expect("Failed to parse first number"),
                parts[1].parse().expect("Failed to parse second number"),
            )
        })
        .collect();

    let mut lists: Vec<Vec<i32>> = contents
        .split_whitespace()
        .filter(|s| s.contains(','))
        .map(|s| {
            s.split(',')
                .map(|num| num.parse().expect("Failed to parse number in list"))
                .collect()
        })
        .collect();

    let mut total = 0;

    while !lists.iter().all(|list| list_is_valid(list, &rules)) {
        (0..lists.len()).for_each(|i| {
            if !list_is_valid(&lists[i], &rules) {
                lists[i] = fix_list(&lists[i], &rules);
                total += lists[i][lists[i].len() / 2]
            }
        });

        // SOLUTION 1
        // for (i, list) in lists.iter().enumerate() {
        //     if list_is_valid(list, &rules) {
        //         // total += list[list.len() / 2];
        //     }
        // }

        println!("TOTAL: {}", total);
    }

    fn fix_list(list: &[i32], rules: &[(i32, i32)]) -> Vec<i32> {
        let mut new_list = list.to_vec();
        let mut i = 0;

        while !list_is_valid(&new_list, rules) {
            let new_rules: Vec<(i32, i32)> = rules
                .iter()
                .filter(|&&rule| rule.0 == new_list[i])
                .cloned()
                .collect();

            let mut swapped = false;
            for (j, &value) in new_list[0..i].iter().enumerate() {
                if new_rules.iter().any(|&rule| rule == (new_list[i], value)) {
                    new_list.swap(i, j);
                    swapped = true;
                    break;
                }
            }

            if !swapped {
                i += 1;
            }
        }

        new_list.to_vec()
    }

    fn list_is_valid(list: &[i32], rules: &[(i32, i32)]) -> bool {
        for (i, &num) in list.iter().enumerate() {
            let new_rules: Vec<(i32, i32)> = rules
                .iter()
                .filter(|&&rule| rule.0 == num)
                .cloned()
                .collect();

            for &j in &list[0..i] {
                if new_rules.iter().any(|&rule| rule == (num, j)) {
                    return false;
                }
            }
        }
        true
    }
}
