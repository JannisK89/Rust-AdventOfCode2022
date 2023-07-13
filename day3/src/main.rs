use day3;
use std::{fs, process};
use utils;

fn main() {
    run_one();
}

fn run_one() {
    let input = fs::read_to_string("./input.txt").unwrap_or_else(|error| {
        eprintln!("{:?}", error);
        process::exit(1);
    });
    let split = utils::split_input(&input);
    let mut letters: Vec<char> = Vec::new();
    let mut total: u32 = 0;

    for line in split {
        if line != "" {
            let letter = day3::find_letter(line);
            match letter {
                Some(letter) => letters.push(letter),
                None => (),
            }
        }
    }

    for letter in &letters {
        let value = day3::calculate_letter_value(letter);
        total += value;
    }
    println!("{total}");
}
