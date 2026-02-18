pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len());
        for bracket in s.as_bytes() {
            let matching_bracket = match bracket {
                b')' => b'(',
                b']' => b'[',
                b'}' => b'{',
                _ => 0,
            };

            if let Some(&last_bracket) = stack.last()
                && last_bracket == &matching_bracket
            {
                stack.pop();
            } else {
                stack.push(bracket);
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            ("()", true),
            ("()[]{}", true),
            ("(]", false),
            ("([])", true),
            ("([)]", false),
        ];

        for test_case in test_cases {
            let (s, expected) = dbg!(test_case);

            let result = Solution::is_valid(s.to_string());

            assert_eq!(expected, result);
        }
    }
}
