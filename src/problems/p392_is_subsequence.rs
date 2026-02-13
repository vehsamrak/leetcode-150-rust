pub struct Solution;

impl Solution {
    // idiomatic two iterators
    pub fn is_subsequence(subsequence: String, haystack: String) -> bool {
        let mut haystack = haystack.bytes();
        subsequence
            .bytes()
            .all(|s| haystack.any(|h| h == s))
    }

    // clasical two pointers
    pub fn is_subsequence_classic(subsequence: String, haystack: String) -> bool {
        let mut pointer = 0;
        let haystack = haystack.as_bytes();
        for letter in subsequence.as_bytes() {
            while pointer < haystack.len() && haystack[pointer] != *letter {
                pointer += 1;
            }

            if pointer >= haystack.len() {
                return false;
            }

            if haystack[pointer] == *letter {
                pointer += 1;
                continue;
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
            ("abc", "ahbgdc", true),
            ("axc", "ahbgdc", false),
            ("aaaaaa", "bbaaaa", false),
        ];

        for test_case in test_cases {
            let (s, t, expected) = test_case;

            let result = Solution::is_subsequence(s.to_string(), t.to_string());
            assert_eq!(expected, result);
            let result = Solution::is_subsequence_classic(s.to_string(), t.to_string());
            assert_eq!(expected, result);
        }
    }
}
