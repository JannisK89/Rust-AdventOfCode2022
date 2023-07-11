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

#[cfg(test)]
mod tests {
    use super::calculate_calories;

    #[test]
    fn returns_highest_calorie_total() {
        let input = vec!["1000", "2000", "", "1000", "5000", "", "3000", "2000", ""];

        let result = calculate_calories(input);
        assert_eq!(result, 6000);
    }
}
