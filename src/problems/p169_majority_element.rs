pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, 0), |(candidate, votes), num| {
                if votes == 0 {
                    (num, 1)
                } else if candidate == num {
                    (candidate, votes + 1)
                } else {
                    (candidate, votes - 1)
                }
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_element() {
        let test_cases = [(vec![3, 2, 3], 3), (vec![2, 2, 1, 1, 1, 2, 2], 2)];

        for test_case in test_cases {
            let nums = test_case.0;
            let expected = test_case.1;

            let result = Solution::majority_element(nums);

            assert_eq!(expected, result);
        }
    }
}
