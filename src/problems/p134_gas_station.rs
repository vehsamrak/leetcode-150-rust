pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut current_tank = 0;
        let mut start = 0;
        let mut gas_cost_total = 0;
        for i in 0..gas.len() {
            let current_gas_cost = gas[i] - cost[i];
            gas_cost_total += current_gas_cost;
            current_tank += current_gas_cost;
            if current_tank < 0 {
                start = i + 1;
                current_tank = 0;
            }
        }

        match gas_cost_total >= 0 {
            true => start as i32,
            false => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            (vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2], 3),
            (vec![2, 3, 4], vec![3, 4, 3], -1),
        ];

        for test_case in test_cases {
            let (gas, cost, expected) = test_case;

            let result = Solution::can_complete_circuit(gas, cost);

            assert_eq!(expected, result);
        }
    }
}
