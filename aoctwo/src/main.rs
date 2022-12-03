fn main() {
    const ROCK: u64 = 1;
    const PAPER: u64 = 2;
    const SCISSORS: u64 = 3;
    const WIN: u64 = 6;
    const LOSS: u64 = 0;
    const DRAW: u64 = 3;

    let file_name = std::env::args()
        .nth(1)
        .expect("please specify an input file");
    let content = std::fs::read_to_string(file_name).expect("failed to read input file");

    // A, X -> rock (1)
    // B, Y -> paper (2)
    // C, Z -> scissors (3)
    // win -> +6
    // loss -> +0
    // draw -> +3
    let total_score: u64 = content
        .trim()
        .split('\n')
        .into_iter()
        .map(|round| {
            println!("{}", round);
            let mut play = round.split(' ');
            let game = (play.next().unwrap(), play.next().unwrap());
            match game {
                ("A", "X") => ROCK + DRAW,
                ("B", "Y") => PAPER + DRAW,
                ("C", "Z") => SCISSORS + DRAW,
                ("C", "X") => ROCK + WIN,
                ("A", "Y") => PAPER + WIN,
                ("B", "Z") => SCISSORS + WIN,
                ("B", "X") => ROCK + LOSS,
                ("C", "Y") => PAPER + LOSS,
                ("A", "Z") => SCISSORS + LOSS,
                (_, _) => 0,
            }
        })
        .sum();

    // X -> lose
    // Y -> draw
    // Z -> win
    let planned_score: u64 = content
        .trim()
        .split('\n')
        .into_iter()
        .map(|round| {
            println!("{}", round);
            let mut play = round.split(' ');
            let game = (play.next().unwrap(), play.next().unwrap());
            match game {
                ("A", "X") => LOSS + SCISSORS,
                ("B", "X") => LOSS + ROCK,
                ("C", "X") => LOSS + PAPER,
                ("A", "Y") => DRAW + ROCK,
                ("B", "Y") => DRAW + PAPER,
                ("C", "Y") => DRAW + SCISSORS,
                ("A", "Z") => WIN + PAPER,
                ("B", "Z") => WIN + SCISSORS,
                ("C", "Z") => WIN + ROCK,
                (_, _) => 0,
            }
        })
        .sum();

    println!("[Part 1]: Total Score: {}", total_score);
    println!("[Part 2]: Planned Score: {}", planned_score);
}
