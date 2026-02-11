pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let needle_bytes = needle.as_bytes();

        haystack
            .as_bytes()
            .windows(needle_bytes.len())
            .position(|w| w == needle_bytes)
            .map_or(-1, |idx| idx as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [("sadbutsad", "sad", 0), ("leetcode", "leeto", -1)];

        for test_case in test_cases {
            let (haystack, needle, expected) = test_case;

            let result = Solution::str_str(haystack.to_string(), needle.to_string());

            assert_eq!(expected, result);
        }
    }
}
