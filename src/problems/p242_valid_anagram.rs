use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut letters = s
            .bytes()
            .fold(HashMap::new(), |mut letters: HashMap<u8, i32>, char| {
                *letters
                    .entry(char)
                    .or_default() += 1;

                letters
            });

        for letter in t.bytes() {
            match letters.get_mut(&letter) {
                Some(l) => {
                    if 0 == *l {
                        return false;
                    }

                    *l -= 1;
                }
                None => return false,
            }
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
            ("anagram", "nagaram", true),
            ("rat", "car", false),
            ("1", "22", false),
            ("22", "1", false),
        ];

        for test_case in test_cases {
            let (s, t, expected) = test_case;

            let result = Solution::is_anagram(s.to_string(), t.to_string());

            assert_eq!(expected, result);
        }
    }
}
