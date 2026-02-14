use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            match map.get(num) {
                Some(&j) => return vec![j as i32, i as i32],
                None => map.insert(target - num, i),
            };
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            (vec![2, 7, 11, 15], 9, vec![0, 1]),
            (vec![3, 2, 4], 6, vec![1, 2]),
            (vec![3, 3], 6, vec![0, 1]),
        ];

        for test_case in test_cases {
            let (nums, target, expected) = test_case;

            let result = Solution::two_sum(nums, target);

            assert_eq!(expected, result);
        }
    }
}
