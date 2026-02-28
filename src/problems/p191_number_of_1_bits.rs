pub struct Solution;

impl Solution {
    // manual bit manipulation
    pub fn hamming_weight(mut n: i32) -> i32 {
        let mut result = 0;
        while n > 0 {
            result += n & 1;
            n >>= 1;
        }

        result
    }

    // idiomatic solution
    pub fn hamming_weight_count_ones(n: i32) -> i32 {
        n.count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let test_cases = dbg!([(11, 3), (128, 1), (2147483645, 30),]);

        for test_case in test_cases {
            let (n, expected) = dbg!(test_case);

            let result = Solution::hamming_weight(n);
            assert_eq!(result, expected);
            let result = Solution::hamming_weight_count_ones(n);
            assert_eq!(result, expected);
        }
    }
}
