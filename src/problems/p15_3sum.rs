use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut result = vec![];
        for (i, &num) in nums.iter().enumerate() {
            if i > 0 && num == nums[i - 1] {
                continue;
            }

            let (mut left, mut right) = (i + 1, nums.len() - 1);
            while left < right {
                let sum = nums[left] + nums[right] + num;
                match sum.cmp(&0) {
                    Ordering::Equal => {
                        while left < right && nums[left] == nums[left + 1] {
                            left += 1;
                        }
                        while left < right && nums[right] == nums[right - 1] {
                            right -= 1;
                        }
                        result.push(vec![nums[left], nums[right], num]);
                        left += 1;
                        right -= 1;
                    }
                    Ordering::Less => left += 1,
                    Ordering::Greater => right -= 1,
                }
            }
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
            (
                vec![-1, 0, 1, 2, -1, -4],
                vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            ),
            (vec![0, 1, 1], vec![]),
            (vec![0, 0, 0], vec![vec![0, 0, 0]]),
            (vec![1, 2, 0, 1, 0, 0, 0, 0], vec![vec![0, 0, 0]]),
        ];

        for test_case in test_cases {
            let (input, mut expected) = dbg!(test_case);
            expected
                .iter_mut()
                .for_each(|x| x.sort_unstable());
            expected.sort_unstable();

            let mut result = Solution::three_sum(input);
            result
                .iter_mut()
                .for_each(|x| x.sort_unstable());
            result.sort_unstable();

            assert_eq!(expected, result);
        }
    }
}
