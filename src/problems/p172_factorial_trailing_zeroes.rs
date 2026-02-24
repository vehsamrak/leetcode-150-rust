pub struct Solution;

impl Solution {
    pub fn trailing_zeroes(mut n: i32) -> i32 {
        let mut result = 0;
        while n > 0 {
            n /= 5;
            result += n;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [(3, 0), (5, 1), (0, 0)];

        for test_case in test_cases {
            let (input, expected) = dbg!(test_case);

            let result = Solution::trailing_zeroes(input);

            assert_eq!(expected, result);
        }
    }
}
