pub struct Solution;

impl Solution {
    pub fn reverse_words(input: String) -> String {
        input
            .split_whitespace()
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            ("the sky is blue", "blue is sky the"),
            ("  hello world  ", "world hello"),
            ("a good   example", "example good a"),
            ("word with 1 digit", "digit 1 with word"),
        ];

        for test_case in test_cases {
            let (input, expected) = test_case;

            let result = Solution::reverse_words(input.to_string());

            assert_eq!(expected, result);
        }
    }
}
