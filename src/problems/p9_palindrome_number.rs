pub struct Solution;

impl Solution {
    pub fn is_palindrome(input: i32) -> bool {
        if input < 0 {
            return false;
        }

        let mut reversed = 0;
        let mut x = input;
        while x != 0 {
            reversed = reversed * 10 + x % 10;
            x /= 10;
        }

        reversed == input
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [(121, true), (-121, false), (10, false)];

        for test_case in test_cases {
            let (input, expected) = dbg!(test_case);

            let result = Solution::is_palindrome(input);
            assert_eq!(expected, result);
        }
    }
}
