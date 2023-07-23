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

pub fn calculate_letter_value(letter: &char) -> u32 {
    let value = *letter as u32;
    let capital = letter.is_uppercase();

    if capital {
        return value - 38;
    } else {
        return value - 96;
    }
}

#[derive(Debug, PartialEq)]
pub struct GroupOfThree<'a> {
    first: &'a str,
    second: &'a str,
    third: &'a str,
}

impl<'a> GroupOfThree<'a> {
    pub fn new(first: &'a str, second: &'a str, third: &'a str) -> GroupOfThree<'a> {
        GroupOfThree {
            first,
            second,
            third,
        }
    }

    pub fn find_common(&self) -> Option<char> {
        for letter in self.first.chars() {
            if self.second.contains(letter) && self.third.contains(letter) {
                return Some(letter);
            }
        }
        None
    }
}

pub fn split_into_groups<'a>(split_text: &'a Vec<&'a str>) -> Vec<GroupOfThree<'a>> {
    let mut iterator = split_text.iter();
    let mut groups: Vec<GroupOfThree> = vec![];
    for _ in 0..split_text.len() / 3 {
        let group = GroupOfThree::new(
            iterator.next().unwrap(),
            iterator.next().unwrap(),
            iterator.next().unwrap(),
        );
        groups.push(group);
    }
    groups
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

    #[test]
    fn returns_letter_available_in_whole_group() {
        let group_one = GroupOfThree::new(
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        );

        let group_two = GroupOfThree::new(
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        );

        let letter_one = group_one.find_common().unwrap();
        let letter_two = group_two.find_common().unwrap();

        assert_eq!(letter_one, 'r');
        assert_eq!(letter_two, 'Z');
    }
    #[test]
    fn returns_groups_of_three() {
        let group_vec = vec![
            "Test1.1", "Test1.2", "Test1.3", "Test2.1", "Test2.2", "Test2.3",
        ];

        let groups = split_into_groups(&group_vec);

        assert_eq!(
            groups[0],
            GroupOfThree {
                first: "Test1.1",
                second: "Test1.2",
                third: "Test1.3"
            }
        );
        assert_eq!(
            groups[1],
            GroupOfThree {
                first: "Test2.1",
                second: "Test2.2",
                third: "Test2.3"
            }
        );
    }
}
