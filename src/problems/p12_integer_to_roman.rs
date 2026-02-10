pub struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut result = String::new();
        let mut remainder = num;
        for (decimal, roman) in [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ] {
            while remainder >= decimal {
                result.push_str(roman);
                remainder -= decimal;
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
        let test_cases = [(3749, "MMMDCCXLIX"), (58, "LVIII"), (1994, "MCMXCIV")];

        for test_case in test_cases {
            let (input, expected) = test_case;

            let result = Solution::int_to_roman(input);

            assert_eq!(expected, result);
        }
    }
}
