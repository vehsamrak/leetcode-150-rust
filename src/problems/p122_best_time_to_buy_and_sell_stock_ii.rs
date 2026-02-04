pub struct Solution;

impl Solution {
    // functional idiomatic solution
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .windows(2)
            .map(|x| (x[1] - x[0]).max(0))
            .sum()
    }

    // classic solution
    pub fn max_profit_2(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        for i in 1..prices.iter().len() {
            if prices[i] > prices[i - 1] {
                max_profit += prices[i] - prices[i - 1];
            }
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_element() {
        let test_cases = [
            (vec![7, 1, 5, 3, 6, 4], 7),
            (vec![1, 2, 3, 4, 5], 4),
            (vec![7, 6, 4, 3, 1], 0),
        ];

        for test_case in test_cases {
            let prices = test_case.0;
            let expected = test_case.1;

            let result = Solution::max_profit(prices.clone());
            let result2 = Solution::max_profit_2(prices);

            assert_eq!(expected, result);
            assert_eq!(expected, result2);
        }
    }
}
