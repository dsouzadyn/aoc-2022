fn occurence_map(line: &str) -> u64 {
    let parts: Vec<&str> = line.split(",").collect();
    let part_a: Vec<u64> = parts[0]
        .split("-")
        .into_iter()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
    let part_b: Vec<u64> = parts[1]
        .split("-")
        .into_iter()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
    let (a, b) = (part_a[0], part_a[1]);
    let (x, y) = (part_b[0], part_b[1]);
    // condition if a and b are smaller than equal to x and y
    let is_valid = (a >= x && b <= y) || (x >= a && y <= b);
    match is_valid {
        true => 1,
        false => 0,
    }
}

fn overlap_map(line: &str) -> u64 {
    let parts: Vec<&str> = line.split(",").collect();
    let part_a: Vec<u64> = parts[0]
        .split("-")
        .into_iter()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
    let part_b: Vec<u64> = parts[1]
        .split("-")
        .into_iter()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
    let (a, b) = (part_a[0], part_a[1]);
    let (x, y) = (part_b[0], part_b[1]);
    // condition if a and b are smaller than equal to x and y
    let max = std::cmp::max(a, x);
    let min = std::cmp::min(b, y);
    let is_valid = max <= min;
    match is_valid {
        true => 1,
        false => 0,
    }
}

fn main() {
    let file_name = std::env::args()
        .nth(1)
        .expect("please specify an input file");
    let content = std::fs::read_to_string(file_name).expect("failed to read input file");

    let occurences: u64 = content.lines().map(|line| occurence_map(line)).sum();

    let overlaps: u64 = content.lines().map(|line| overlap_map(line)).sum();

    println!("[Part 1]: {:?}", occurences);
    println!("[Part 2]: {:?}", overlaps);
}
