use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        const MAX_ENGLISH_LETTERS: usize = 26;
        let mut map: HashMap<u8, i32> = HashMap::with_capacity(MAX_ENGLISH_LETTERS);
        magazine
            .bytes()
            .for_each(|x| *map.entry(x).or_default() += 1);

        ransom_note
            .bytes()
            .all(|x| match map.get_mut(&x) {
                Some(count) if *count > 0 => {
                    *count -= 1;
                    true
                }
                _ => false,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [("a", "b", false), ("aa", "ab", false), ("aa", "aab", true)];

        for test_case in test_cases {
            let (note, magazine, expected) = test_case;

            let result = Solution::can_construct(note.to_string(), magazine.to_string());
            assert_eq!(expected, result);
        }
    }
}
