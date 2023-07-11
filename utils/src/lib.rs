pub fn split_input<'a>(input: &'a str) -> Vec<&'a str> {
    let split_input: Vec<&str> = input.split(&['\n', '\r']).collect();
    split_input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_input_returns_vector_with_values() {
        let input = "\
1432
2415
5235

5433
4422
";

        let result = split_input(input);
        assert_eq!(result, vec!["1432", "2415", "5235", "", "5433", "4422", ""]);
    }
}
