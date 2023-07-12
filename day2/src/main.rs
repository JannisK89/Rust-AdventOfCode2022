mod sol;
use sol::{part_one, part_two};
use std::{fs, process};
use utils;

fn main() {
    run_part_one();
    run_part_two();
}

fn run_part_one() {
    let input = fs::read_to_string("./input.txt").unwrap_or_else(|error| {
        eprintln!("{:?}", error);
        process::exit(1);
    });
    let split = utils::split_input(&input);
    let mut total_score = 0;
    for line in split {
        if line != "" {
            let matchup = part_one::parse_matchup(line).unwrap_or_else(|| {
                eprintln!(
                    "Could not parse match up:W
                          , check your input file"
                );
                process::exit(1);
            });
            let score = part_one::calculate_score(&matchup);
            total_score += score;
        }
    }
    println!("Score: {total_score}");
}

fn run_part_two() {
    let input = fs::read_to_string("./input.txt").unwrap_or_else(|error| {
        eprintln!("{:?}", error);
        process::exit(1);
    });
    let split = utils::split_input(&input);
    let mut total_score = 0;
    for line in split {
        if line != "" {
            let matchup = part_two::parse_matchup(line).unwrap_or_else(|| {
                eprintln!(
                    "Could not parse match up:W
                          , check your input file"
                );
                process::exit(1);
            });
            let score = part_two::calculate_score(&matchup);
            total_score += score;
        }
    }
    println!("Score: {total_score}");
}
