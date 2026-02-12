pub struct Solution;

impl Solution {
    // mathematical optimal solution
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        let mut result = String::with_capacity(s.len());
        let circle = 2 * (num_rows - 1);
        let input = s.as_bytes();

        if num_rows == 1 {
            return s;
        }

        for row in 0..num_rows {
            let mut j = row;
            while j < input.len() {
                result.push(input[j] as char);

                if row != 0 && row != num_rows - 1 {
                    let diagonal_index = j + circle - 2 * row;
                    if diagonal_index < input.len() {
                        result.push(input[diagonal_index] as char);
                    }
                }

                j += circle;
            }
        }

        result
    }

    // clear but memory inefficient solution
    pub fn convert_simulation(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        let mut result = vec![String::new(); num_rows];
        let mut current_row = 0;
        let mut direction_down = true;
        for letter in s.chars() {
            result[current_row].push(letter);

            if current_row == 0 || current_row == num_rows - 1 {
                direction_down = !direction_down;
            }

            current_row = match direction_down {
                true => current_row - 1,
                false => current_row + 1,
            }
        }

        result.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            ("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR"),
            ("PAYPALISHIRING", 4, "PINALSIGYAHRPI"),
            ("A", 1, "A"),
        ];

        for test_case in test_cases {
            let (input, rows, expected) = test_case;

            let result = Solution::convert(input.to_string(), rows);
            assert_eq!(expected, result);
            let result = Solution::convert_simulation(input.to_string(), rows);
            assert_eq!(expected, result);
        }
    }
}
