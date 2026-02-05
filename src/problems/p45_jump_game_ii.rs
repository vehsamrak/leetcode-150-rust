pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut border = 0;
        let mut distance = 0;
        let mut jumps = 0;

        for (i, &num) in nums
            .iter()
            .take(nums.len() - 1)
            .enumerate()
        {
            distance = distance.max(i + num as usize);

            if i == border {
                border = distance;
                jumps += 1;
            }
        }

        jumps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [(vec![2, 3, 1, 1, 4], 2), (vec![2, 3, 0, 1, 4], 2)];

        for test_case in test_cases {
            let nums = test_case.0;
            let expected = test_case.1;

            let result = Solution::jump(nums);

            assert_eq!(expected, result);
        }
    }
}
