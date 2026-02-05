pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut [i32], val: i32) -> usize {
        let (mut i, mut j) = (0, nums.len());

        while i < j {
            if nums[i] == val {
                j -= 1;
                if nums[j] == val {
                    continue;
                }

                nums.swap(i, j);
                continue;
            }

            i += 1;
        }

        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            (vec![3, 2, 2, 3], 3, 2, vec![2, 2]),
            (vec![0, 1, 2, 2, 3, 0, 4, 2], 2, 5, vec![0, 1, 3, 0, 4]),
        ];

        for test_case in test_cases {
            let mut nums = test_case.0;
            let val = test_case.1;
            let expected_k = test_case.2;

            let k = Solution::remove_element(&mut nums, val);

            for i in nums.iter().take(k) {
                assert_ne!(&val, i);
            }
            assert_eq!(expected_k, k);
        }
    }
}
