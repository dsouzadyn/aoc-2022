use std::collections::HashSet;

fn main() {
    let file_name = std::env::args()
        .nth(1)
        .expect("please specify an input file");
    let content = std::fs::read_to_string(file_name).expect("failed to read input file");

    let priority_sum: u64 = content
        .trim()
        .split('\n')
        .map(|items| {
            let compartment_data = items.split_at(items.len() / 2);
            let a: HashSet<char> =
                HashSet::from_iter::<Vec<char>>(compartment_data.0.chars().collect());
            let b: HashSet<char> =
                HashSet::from_iter::<Vec<char>>(compartment_data.1.chars().collect());
            a.intersection(&b)
                .map(|&c| {
                    if c.is_lowercase() {
                        c as u64 - '`' as u64
                    } else {
                        c as u64 - '&' as u64
                    }
                })
                .sum::<u64>()
        })
        .sum();

    println!("[Part 1]: {}", priority_sum);
}