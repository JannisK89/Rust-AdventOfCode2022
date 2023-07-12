use std::collections::HashMap;

pub struct Matchup {
    opponent: Hand,
}

impl Matchup {
    fn new(opponent: Hand) -> Matchup {
        Matchup { opponent }
    }
}

#[derive(PartialEq, Debug, Eq, Hash)]
enum Hand {
    Rock(MatchResult),
    Paper(MatchResult),
    Scissors(MatchResult),
}

#[derive(PartialEq, Debug, Hash, Eq)]
enum MatchResult {
    Win,
    Tie,
    Loss,
}

pub fn parse_matchup(matchup: &str) -> Option<Matchup> {
    if matchup.len() > 1 {
        let split: Vec<&str> = matchup.split_whitespace().collect();

        let result = match split[1] {
            "X" => Some(MatchResult::Loss),
            "Y" => Some(MatchResult::Tie),
            "Z" => Some(MatchResult::Win),
            _ => None,
        }
        .expect("Input error");

        let opponent = match split[0] {
            "A" => Some(Hand::Rock(result)),
            "B" => Some(Hand::Paper(result)),
            "C" => Some(Hand::Scissors(result)),
            _ => None,
        }
        .expect("Input error");

        return Some(Matchup::new(opponent));
    }
    None
}

pub fn calculate_score(matchup: &Matchup) -> u32 {
    let mut score_list = HashMap::new();
    score_list.insert(Hand::Rock(MatchResult::Win), 8);
    score_list.insert(Hand::Rock(MatchResult::Loss), 3);
    score_list.insert(Hand::Rock(MatchResult::Tie), 4);
    score_list.insert(Hand::Paper(MatchResult::Win), 9);
    score_list.insert(Hand::Paper(MatchResult::Loss), 1);
    score_list.insert(Hand::Paper(MatchResult::Tie), 5);
    score_list.insert(Hand::Scissors(MatchResult::Win), 7);
    score_list.insert(Hand::Scissors(MatchResult::Loss), 2);
    score_list.insert(Hand::Scissors(MatchResult::Tie), 6);

    score_list[&matchup.opponent]
}
#[test]
fn returns_matchup() {
    let input_one = "A Y";
    let input_two = "B X";
    let input_three = "C Z";

    let result_one = parse_matchup(input_one).unwrap();
    let result_two = parse_matchup(input_two).unwrap();
    let result_three = parse_matchup(input_three).unwrap();

    assert_eq!(result_one.opponent, Hand::Rock(MatchResult::Tie) );
    assert_eq!(result_two.opponent, Hand::Paper(MatchResult::Loss));
    assert_eq!(result_three.opponent, Hand::Scissors(MatchResult::Win));
}

#[test]
fn returns_score() {
    let input_win = Matchup {
        opponent: Hand::Rock(MatchResult::Win),
    };
    let input_loss = Matchup {
        opponent: Hand::Scissors(MatchResult::Loss),
    };
    let input_tie = Matchup {
        opponent: Hand::Paper(MatchResult::Tie),
    };

    let result_win = calculate_score(&input_win);
    let result_loss = calculate_score(&input_loss);
    let result_tie = calculate_score(&input_tie);

    assert_eq!(result_win, 8);
    assert_eq!(result_loss, 2);
    assert_eq!(result_tie, 5);
}
