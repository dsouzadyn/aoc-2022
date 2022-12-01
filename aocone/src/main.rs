use std::collections::BinaryHeap;

fn main() {
    let file_name = std::env::args().nth(1).expect("please specify an input file");

    let content = std::fs::read_to_string(file_name).expect("failed to read input file");
    
    // Part One
    let max_calories = content.trim()
    .split("\n\n").into_iter()
    .map(
        |x| x.split('\n').into_iter()
        .map(|y| y.parse::<u64>().unwrap()
    )
    .sum::<u64>())
    .max().unwrap();

    println!("Part One: {}", max_calories);

    // Part Two
    let mut heap: BinaryHeap<u64> = content.trim()
    .split("\n\n").into_iter()
    .map(
        |x| x.split('\n').into_iter()
        .map(|y| y.parse::<u64>().unwrap()
    )
    .sum::<u64>()).collect();

    let three_sum = heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap();
    println!("Part Two: {}", three_sum)

}
