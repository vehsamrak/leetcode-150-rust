use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let length = s.len();
        let s = s.as_bytes();
        let t = t.as_bytes();

        let (mut s_map, mut t_map) = (
            HashMap::<u8, u8>::with_capacity(length),
            HashMap::<u8, u8>::with_capacity(length),
        );
        for i in 0..length {
            let (s_letter, t_letter) = (s[i], t[i]);
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
        }
    }
}
