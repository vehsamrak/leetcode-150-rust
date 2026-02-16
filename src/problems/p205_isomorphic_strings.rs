use std::collections::HashMap;

pub struct Solution;

impl Solution {
    // last occurrence approach
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let mut last_occurrences_s = [0usize; 256];
        let mut last_occurrences_t = [0usize; 256];

        for i in 0..s.len() {
            let s_letter = s[i] as usize;
            let t_letter = t[i] as usize;

            if last_occurrences_s[s_letter] != last_occurrences_t[t_letter] {
                return false;
            }

            last_occurrences_s[s_letter] = i + 1;
            last_occurrences_t[t_letter] = i + 1;
        }

        true
    }

    // two hash maps approach
    pub fn is_isomorphic_two_maps(s: String, t: String) -> bool {
        let length = s.len();
        let s = s.as_bytes();
        let t = t.as_bytes();

        let (mut s_map, mut t_map) = (
            HashMap::<u8, u8>::with_capacity(length),
            HashMap::<u8, u8>::with_capacity(length),
        );

        for (&s_letter, &t_letter) in s.iter().zip(t.iter()) {
            if let Some(&letter) = s_map.get(&s_letter)
                && letter != t_letter
            {
                return false;
            } else {
                s_map.insert(s_letter, t_letter);
            }

            if let Some(&letter) = t_map.get(&t_letter)
                && letter != s_letter
            {
                return false;
            } else {
                t_map.insert(t_letter, s_letter);
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
            ("egg", "add", true),
            ("f11", "b23", false),
            ("paper", "title", true),
        ];

        for test_case in test_cases {
            let (s, t, expected) = dbg!(test_case);

            let result = Solution::is_isomorphic(s.to_string(), t.to_string());
            assert_eq!(expected, result);

            let result = Solution::is_isomorphic_two_maps(s.to_string(), t.to_string());
            assert_eq!(expected, result);
        }
    }
}
