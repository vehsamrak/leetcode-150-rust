pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut [i32], k: i32) {
        let k = k as usize % nums.len();

        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_element() {
        let test_cases = [
            (vec![1, 2, 3, 4, 5, 6, 7], 3, vec![5, 6, 7, 1, 2, 3, 4]),
            (vec![-1, -100, 3, 99], 2, vec![3, 99, -1, -100]),
        ];

        for test_case in test_cases {
            let mut nums = test_case.0;
            let k = test_case.1;
            let expected_nums = test_case.2;

            Solution::rotate(&mut nums, k);

            assert_eq!(expected_nums, nums);
        }
    }
}
