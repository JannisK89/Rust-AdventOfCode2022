use std::fs;
use std::process;
use utils;

fn main() {
    run_day1();
    run_day2();
}

fn run_day1() {
    let input = fs::read_to_string("./input.txt").unwrap_or_else(|error| {
        eprintln!("{:?}", error);
        process::exit(1);
    });
    let split = utils::split_input(&input);
    let total = day1::calculate_calories(split);
    println!("Day 1, part 1: {total}");
}

fn run_day2() {
    let input = fs::read_to_string("./input.txt").unwrap_or_else(|error| {
        eprintln!("{:?}", error);
        process::exit(1);
    });
    let split = utils::split_input(&input);
    let total = day1::calculate_top_three_calories(split);
    println!("Day 1, part 2: {total}");
}
