pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut [i32]) -> usize {
        let mut j = 0;
        for i in 0..nums.len() {
            if i == 0 || nums[i - 1] != nums[i] {
                nums[j] = nums[i];
                j += 1;
            }
        }

        j
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            (vec![1, 1, 2], 2, vec![1, 2]),
            (vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], 5, vec![0, 1, 2, 3, 4]),
        ];

        for test_case in test_cases {
            let mut nums = test_case.0;
            let expected_result = test_case.1 as usize;
            let expected_nums = test_case.2;

            let result = Solution::remove_duplicates(&mut nums);

            assert_eq!(expected_result, result);
            assert_eq!(expected_nums, &nums[..expected_result]);
        }
    }
}
