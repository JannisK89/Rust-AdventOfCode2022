#[derive(PartialEq, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Debug)]
enum MatchResult {
    Win,
    Tie,
    Loss,
}

pub struct Matchup {
    opponent: Hand,
    you: Hand,
}

impl Matchup {
    fn new(opponent: Hand, you: Hand) -> Matchup {
        Matchup { opponent, you }
    }
}

pub fn parse_matchup(matchup: &str) -> Option<Matchup> {
    if matchup.len() > 1 {
        let split: Vec<&str> = matchup.split_whitespace().collect();

        let opponent = match split[0] {
            "A" => Some(Hand::Rock),
            "B" => Some(Hand::Paper),
            "C" => Some(Hand::Scissors),
            _ => None,
        }
        .expect("Input error");

        let you = match split[1] {
            "X" => Some(Hand::Rock),
            "Y" => Some(Hand::Paper),
            "Z" => Some(Hand::Scissors),
            _ => None,
        }
        .expect("Input error");

        return Some(Matchup::new(opponent, you));
    }
    None
}

fn calculate_winner(matchup: &Matchup) -> MatchResult {
    let score = match matchup {
        Matchup {
            opponent: Hand::Rock,
            you: Hand::Rock,
        }
        | Matchup {
            opponent: Hand::Paper,
            you: Hand::Paper,
        }
        | Matchup {
            opponent: Hand::Scissors,
            you: Hand::Scissors,
        } => MatchResult::Tie,
        Matchup {
            opponent: Hand::Rock,
            you: Hand::Scissors,
        }
        | Matchup {
            opponent: Hand::Paper,
            you: Hand::Rock,
        }
        | Matchup {
            opponent: Hand::Scissors,
            you: Hand::Paper,
        } => MatchResult::Loss,
        Matchup {
            opponent: Hand::Rock,
            you: Hand::Paper,
        }
        | Matchup {
            opponent: Hand::Paper,
            you: Hand::Scissors,
        }
        | Matchup {
            opponent: Hand::Scissors,
            you: Hand::Rock,
        } => MatchResult::Win,
    };
    score
}

pub fn calculate_score(matchup: &Matchup) -> u64 {
    let score = calculate_winner(&matchup);
    let score = match score {
        MatchResult::Win => 6,
        MatchResult::Tie => 3,
        MatchResult::Loss => 0,
    };

    let hand_score = match matchup.you {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    };

    score + hand_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_matchup() {
        let input_one = "A Y";
        let input_two = "B X";
        let input_three = "C Z";

        let result_one = parse_matchup(input_one).unwrap();
        let result_two = parse_matchup(input_two).unwrap();
        let result_three = parse_matchup(input_three).unwrap();

        assert_eq!(result_one.opponent, Hand::Rock);
        assert_eq!(result_one.you, Hand::Paper);
        assert_eq!(result_two.opponent, Hand::Paper);
        assert_eq!(result_two.you, Hand::Rock);
        assert_eq!(result_three.opponent, Hand::Scissors);
        assert_eq!(result_three.you, Hand::Scissors);
    }

    #[test]
    fn returns_correct_winner() {
        let input_win = Matchup {
            opponent: Hand::Rock,
            you: Hand::Paper,
        };
        let input_loss = Matchup {
            opponent: Hand::Scissors,
            you: Hand::Paper,
        };
        let input_tie = Matchup {
            opponent: Hand::Rock,
            you: Hand::Rock,
        };

        let result_win = calculate_winner(&input_win);
        let result_loss = calculate_winner(&input_loss);
        let result_tie = calculate_winner(&input_tie);

        assert_eq!(result_win, MatchResult::Win);
        assert_eq!(result_loss, MatchResult::Loss);
        assert_eq!(result_tie, MatchResult::Tie);
    }

    #[test]
    fn returns_correct_score() {
        let input_one = Matchup {
            opponent: Hand::Paper,
            you: Hand::Paper,
        };
        let input_two = Matchup {
            opponent: Hand::Paper,
            you: Hand::Scissors,
        };
        let input_three = Matchup {
            opponent: Hand::Rock,
            you: Hand::Scissors,
        };

        let results_one = calculate_score(&input_one);
        let results_two = calculate_score(&input_two);
        let results_three = calculate_score(&input_three);

        assert_eq!(results_one, 5);
        assert_eq!(results_two, 9);
        assert_eq!(results_three, 3);
    }
}
