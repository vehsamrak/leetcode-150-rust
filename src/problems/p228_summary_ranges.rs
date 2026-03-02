use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return vec![];
        }

        let mut result: Vec<(i32, i32)> = Vec::new();
        let mut start = nums[0];
        let mut end = start;
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                continue;
            }

            if nums[i] == end + 1 {
                end = nums[i];
            } else {
                result.push((start, end));
                start = nums[i];
                end = nums[i];
            }
        }

        result.push((start, end));

        result
            .into_iter()
            .map(|x| match &x.0.cmp(&x.1) {
                Ordering::Equal => x.1.to_string(),
                _ => format!("{}->{}", x.0, x.1),
            })
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            (vec![4, 5], vec!["4->5"]),
            (vec![0, 1, 2, 4, 5, 7], vec!["0->2", "4->5", "7"]),
            (vec![0, 2, 3, 4, 6, 8, 9], vec!["0", "2->4", "6", "8->9"]),
            (vec![], vec![]),
            (vec![0], vec!["0"]),
            (vec![1, 2, 3, 4, 5, 6, 7, 8], vec!["1->8"]),
        ];

        for test_case in test_cases {
            let (nums, expected) = dbg!(test_case);
            let expected: Vec<String> = expected
                .iter()
                .map(|x| x.to_string())
                .collect();

            let result = Solution::summary_ranges(nums);

            assert_eq!(expected, result);
        }
    }
}
