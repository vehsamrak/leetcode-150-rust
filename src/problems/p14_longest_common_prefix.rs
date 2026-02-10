pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.iter()
            .skip(1)
            .fold(strs[0].clone(), |prefix, string| {
                prefix
                    .chars()
                    .zip(string.chars())
                    .take_while(|(a, b)| a == b)
                    .map(|(a, _)| a)
                    .collect()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            (vec!["flower", "flow", "flight"], "fl"),
            (vec!["dog", "racecar", "car"], ""),
        ];

        for test_case in test_cases {
            let (input, expected) = test_case;

            let result = Solution::longest_common_prefix(
                input
                    .into_iter()
                    .map(String::from)
                    .collect(),
            );

            assert_eq!(expected, result);
        }
    }
}
