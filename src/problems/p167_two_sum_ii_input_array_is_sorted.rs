use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0, numbers.len() - 1);
        while left < right {
            match (numbers[left] + numbers[right]).cmp(&target) {
                Ordering::Equal => return vec![left as i32 + 1, right as i32 + 1],
                Ordering::Greater => right -= 1,
                Ordering::Less => left += 1,
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            (vec![2, 7, 11, 15], 9, vec![1, 2]),
            (vec![2, 3, 4], 6, vec![1, 3]),
            (vec![-1, 0], -1, vec![1, 2]),
        ];

        for test_case in test_cases {
            let (numbers, target, expected): (Vec<i32>, i32, Vec<i32>) = test_case;

            let result = Solution::two_sum(numbers, target);

            assert_eq!(expected, result);
        }
    }
}
