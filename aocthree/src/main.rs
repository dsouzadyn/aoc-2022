use std::collections::HashSet;

fn compute_val(c: char) -> u64 {
    if c.is_lowercase() {
        c as u64 - '`' as u64
    } else {
        c as u64 - '&' as u64
    }
}

fn main() {
    let file_name = std::env::args()
        .nth(1)
        .expect("please specify an input file");
    let content = std::fs::read_to_string(file_name).expect("failed to read input file");

    let priority_sum: u64 = content
        .lines()
        .map(|items| {
            let (compartment_one, compartment_two) = items.split_at(items.len() / 2);
            let a: HashSet<char> = HashSet::from_iter(compartment_one.chars());
            let b: HashSet<char> = HashSet::from_iter(compartment_two.chars());
            a.intersection(&b).map(|&c| compute_val(c)).sum::<u64>()
        })
        .sum();

    let lines: Vec<&str> = content.lines().collect();
    let chunk_sum: u64 = lines
        .chunks(3)
        .map(|chunk| {
            let a: HashSet<char> = HashSet::from_iter(chunk[0].chars());
            let b: HashSet<char> = HashSet::from_iter(chunk[1].chars());
            let c: HashSet<char> = HashSet::from_iter(chunk[2].chars());
            let intermediate: HashSet<char> = HashSet::from_iter(a.intersection(&b).cloned());
            c.intersection(&intermediate)
                .map(|&c| compute_val(c))
                .sum::<u64>()
        })
        .sum();

    println!("[Part 1]: {}", priority_sum);
    println!("[Part 2]: {}", chunk_sum);
}
