pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.chars()
            .rev()
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            ("Hello World", 5),
            ("   fly me   to   the moon  ", 4),
            ("luffy is still joyboy", 6),
            ("", 0),
            ("     ", 0),
        ];

        for test_case in test_cases {
            let (input, expected) = test_case;

            let result = Solution::length_of_last_word(input.to_string());

            assert_eq!(expected, result);
        }
    }
}
