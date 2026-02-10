pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut previous = 0;

        for letter in s.chars().rev() {
            let current = match letter {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };

            if current < previous {
                result -= current;
            } else {
                result += current;
            }
            previous = current;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [("III", 3), ("LVIII", 58), ("MCMXCIV", 1994)];

        for test_case in test_cases {
            let (roman, expected) = test_case;

            let result = Solution::roman_to_int(roman.to_string());

            assert_eq!(expected, result);
        }
    }
}
