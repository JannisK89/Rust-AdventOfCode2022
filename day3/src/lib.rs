pub fn find_letter(line: &str) -> Option<char> {
    let length = line.len() / 2;
    let first_half = &line[0..length];
    let second_half = &line[length..];

    for letter in first_half.chars() {
        if second_half.contains(letter) {
            return Some(letter);
        }
    }
    None
}

pub fn calculate_letter_value(letter:&char) -> u32 {
        let value = *letter as u32;
        let capital = letter.is_uppercase();

        if capital {
           return value-38; 
        } else {
            return value-96
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_reoccuring_letter() {
        let test_strings = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        let reoccuring_letters = vec!['p', 'L', 'P', 'v', 't', 's'];

        for index in 0..test_strings.len() {
            let letter = find_letter(test_strings[index]).unwrap();
            assert_eq!(letter, reoccuring_letters[index]);
        }
    }

    #[test]
    fn returns_correct_letter_score() {
        let reoccuring_letters = vec!['p', 'L', 'P', 'v', 't', 's'];
        let score_vec = vec![16, 38, 42, 22, 20, 19];

        for index in 0..reoccuring_letters.len() {
            let score = calculate_letter_value(&reoccuring_letters[index]);
            assert_eq!(score, score_vec[index]);
        }
    }
}
