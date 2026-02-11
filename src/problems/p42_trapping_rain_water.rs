pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let (mut left_max, mut right_max) = (0, 0);
        let mut result = 0;

        while left < right {
            if height[left] < height[right] {
                left_max = left_max.max(height[left]);
                result += left_max - height[left];
                left += 1;
            } else {
                right_max = right_max.max(height[right]);
                result += right_max - height[right];
                right -= 1;
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
            (vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6),
            (vec![4, 2, 0, 3, 2, 5], 9),
        ];

        for test_case in test_cases {
            let (input, expected) = test_case;

            let result = Solution::trap(input);

            assert_eq!(expected, result);
        }
    }
}
