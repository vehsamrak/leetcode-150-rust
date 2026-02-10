pub struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candies = vec![1; ratings.len()];

        for (i, window) in ratings.windows(2).enumerate() {
            if window[1] > window[0] {
                candies[i + 1] = candies[i] + 1;
            }
        }

        for (i, window) in ratings
            .windows(2)
            .enumerate()
            .rev()
        {
            if window[0] > window[1] {
                candies[i] = candies[i].max(candies[i + 1] + 1);
            }
        }

        candies.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [(vec![1, 0, 2], 5), (vec![1, 2, 2], 4), (vec![1, 2, 2], 4)];

        for test_case in test_cases {
            let (input, expected) = test_case;

            let result = Solution::candy(input);

            assert_eq!(expected, result);
        }
    }
}
