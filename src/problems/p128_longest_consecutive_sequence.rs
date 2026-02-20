use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let unique_nums: HashSet<i32> = nums.into_iter().collect();

        unique_nums
            .iter()
            .filter(|&x| !unique_nums.contains(&(x - 1)))
            .fold(0, |max_streak, &x| {
                let streak = (x..)
                    .take_while(|n| unique_nums.contains(n))
                    .count();

                max_streak.max(streak as i32)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            (vec![100, 4, 200, 1, 3, 2], 4),
            (vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1], 9),
        ];

        for test_case in test_cases {
            let (nums, expected) = dbg!(test_case);

            let result = Solution::longest_consecutive(nums);

            assert_eq!(expected, result);
        }
    }
}
