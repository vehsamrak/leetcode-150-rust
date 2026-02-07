pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];

        for i in 1..nums.len() {
            result[i] = result[i - 1] * nums[i - 1];
        }

        let mut postfix = 1;
        for i in (0..nums.len()).rev() {
            result[i] *= postfix;
            postfix *= nums[i];
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            (vec![1, 2, 3, 4], vec![24, 12, 8, 6]),
            (vec![-1, 1, 0, -3, 3], vec![0, 0, 9, 0, 0]),
        ];

        for test_case in test_cases {
            let input = test_case.0;
            let expected = test_case.1;

            let result = Solution::product_except_self(input);

            assert_eq!(expected, result);
        }
    }
}
