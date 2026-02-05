pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_distance = 0;

        for (i, &num) in nums
            .iter()
            .enumerate()
        {
            if i > max_distance {
                return false;
            }

            max_distance = max_distance.max(i + num as usize);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [(vec![2, 3, 1, 1, 4], true), (vec![3, 2, 1, 0, 4], false)];

        for test_case in test_cases {
            let nums = test_case.0;
            let expected = test_case.1;

            let result = Solution::can_jump(nums);

            assert_eq!(expected, result);
        }
    }
}
