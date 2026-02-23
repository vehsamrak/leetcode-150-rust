pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for i in (0..digits.len()).rev() {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }

            digits[i] = 0;
        }

        digits.resize(digits.len() + 1, 0);
        digits[0] = 1;
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            (vec![1, 2, 3], vec![1, 2, 4]),
            (vec![4, 3, 2, 1], vec![4, 3, 2, 2]),
            (vec![9], vec![1, 0]),
            (vec![9, 9], vec![1, 0, 0]),
        ];

        for test_case in test_cases {
            let (input, expected) = test_case;

            let result = Solution::plus_one(input);

            assert_eq!(expected, result);
        }
    }
}
