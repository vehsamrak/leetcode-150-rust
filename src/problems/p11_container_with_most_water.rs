use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_volume = 0;
        let (mut left, mut right) = (0, height.len() - 1);

        while left < right {
            let min_height = height[left].min(height[right]);
            let width = right - left;
            let volume = min_height as usize * width;

            max_volume = max_volume.max(volume);

            match height[left].cmp(&height[right]) {
                Ordering::Less => left += 1,
                _ => right -= 1,
            }
        }

        max_volume as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [(vec![1, 8, 6, 2, 5, 4, 8, 3, 7], 49), (vec![1, 1], 1)];

        for test_case in test_cases {
            let (input, expected) = test_case;

            let result = Solution::max_area(input);

            assert_eq!(expected, result);
        }
    }
}
