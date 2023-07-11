pub fn calculate_calories(calorie_vector: Vec<&str>) -> i32 {
    let mut largest = 0;
    let mut total = 0;
    for value in calorie_vector {
        if value != "" {
            let parsed_value: i32 = value.parse().unwrap_or(0);
            total += parsed_value;
        } else {
            if total > largest {
                largest = total;
            }
            total = 0;
        }
    }
    largest
}

pub fn calculate_top_three_calories(calorie_vector: Vec<&str>) -> i32 {
    let mut largest = Vec::new();
    let mut total = 0;
    for value in calorie_vector {
        if value != "" {
            let parsed_value: i32 = value.parse().unwrap_or(0);
            total += parsed_value;
        } else {
            if largest.len() < 3 {
                largest.push(total);
            } else {
                largest.sort();
                if largest[0] < total {
                    largest[0] = total;
                }
            }
            total = 0;
        }
    }
    largest[0] + largest[1] + largest[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_highest_calorie_total() {
        let input = vec!["1000", "2000", "", "1000", "5000", "", "3000", "2000", ""];

        let result = calculate_calories(input);
        assert_eq!(result, 6000);
    }

    #[test]
    fn returns_top_three_total() {
        let input = vec![
            "1000", "2000", "", "1000", "5000", "", "3000", "2000", "", "1000", "", "10000", "",
        ];

        let result = calculate_top_three_calories(input);
        assert_eq!(result, 21000);
    }
}
