use std::collections::HashMap;

pub struct Solution;

impl Solution {
    // idiomatic iterator with map insert post-check
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::with_capacity(nums.len());

        nums.iter()
            .enumerate()
            .any(|(i, &num)| {
                map.insert(num, i)
                    .is_some_and(|x| i - x <= k as usize)
            })
    }

    // classical approach
    pub fn contains_nearby_duplicate_classic(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (i, num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(num)
                && i.abs_diff(j) <= k as usize
            {
                return true;
            }

            map.insert(*num, i);
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            (vec![1, 2, 3, 1], 3, true),
            (vec![1, 0, 1, 1], 1, true),
            (vec![1, 2, 3, 1, 2, 3], 2, false),
        ];

        for test_case in test_cases {
            let (nums, k, expected) = test_case;

            let result = Solution::contains_nearby_duplicate(nums.clone(), k);
            assert_eq!(expected, result);
            let result = Solution::contains_nearby_duplicate_classic(nums, k);
            assert_eq!(expected, result);
        }
    }
}
