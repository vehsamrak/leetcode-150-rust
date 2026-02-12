pub struct Solution;

impl Solution {
    pub fn is_palindrome(input: String) -> bool {
        let input = input.as_bytes();
        let (mut left, mut right) = (0, input.len() - 1);
        while left < right {
            while left < right && !input[left].is_ascii_alphanumeric() {
                left += 1;
            }

            while left < right && !input[right].is_ascii_alphanumeric() {
                right -= 1;
            }

            if left < right && !input[left].eq_ignore_ascii_case(&input[right]) {
                return false;
            }

            left += 1;
            right = right.saturating_sub(0);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            ("A man, a plan, a canal: Panama", true),
            ("race a car", false),
            (" ", true),
            ("a.", false),
        ];

        for test_case in test_cases {
            let (input, expected) = test_case;

            let result = Solution::is_palindrome(input.to_string());
            assert_eq!(expected, result);
        }
    }
}
