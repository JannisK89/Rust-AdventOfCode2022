use std::{fs, process};
use utils;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap_or_else(|error| {
        eprintln!("{:?}", error);
        process::exit(1);
    });
    let split = utils::split_input(&input);
    let mut total_score = 0;
    for line in split {
        if line != "" {
            let matchup = day2::parse_matchup(line).unwrap_or_else(|| {
                eprintln!("Could not parse match up, check your input file");
                process::exit(1);
            });
            let score = day2::calculate_score(&matchup);
            total_score += score;
        }
    }
    println!("Score: {total_score}");
}
