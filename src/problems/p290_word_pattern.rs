use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let pattern = pattern.as_bytes();
        let words: Vec<&str> = s
            .split_ascii_whitespace()
            .collect();

        if pattern.len() != words.len() {
            return false;
        }

        let mut pattern_map: HashMap<u8, String> = HashMap::with_capacity(pattern.len());
        let mut word_map: HashMap<String, u8> = HashMap::with_capacity(words.len());
        for (letter, &word) in pattern
            .iter()
            .zip(words.iter())
        {
            if let Some(w) = pattern_map.get(letter)
                && w != word
            {
                return false;
            }

            pattern_map.insert(*letter, word.to_string());

            if let Some(l) = word_map.get(word)
                && l != letter
            {
                return false;
            }

            word_map.insert(word.to_string(), *letter);
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
            ("abba", "dog cat cat dog", true),
            ("abba", "dog cat cat fish", false),
            ("aaaa", "dog cat cat dog", false),
            ("abba", "dog dog dog dog", false),
            ("aaa", "aa aa aa aa", false),
        ];

        for test_case in test_cases {
            let (pattern, s, expected) = dbg!(test_case);

            let result = Solution::word_pattern(pattern.to_string(), s.to_string());

            assert_eq!(expected, result);
        }
    }
}
