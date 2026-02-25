use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = vec![HashSet::new(); 9];
        let mut columns = vec![HashSet::new(); 9];
        let mut cubes = vec![HashSet::new(); 9];
        for r in 0..9 {
            for c in 0..9 {
                let cell = board[r][c];
                if cell == '.' {
                    continue;
                }

                let cube_row = r / 3;
                let cube_column = c / 3;
                let cube_index = cube_row * 3 + cube_column;

                if !rows[r].insert(cell)
                    || !columns[c].insert(cell)
                    || !cubes[cube_index].insert(cell)
                {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            (
                vec![
                    vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                    vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                    vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                    vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                    vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                    vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                    vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                    vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                    vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
                ],
                true,
            ),
            (
                vec![
                    vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                    vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                    vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                    vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                    vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                    vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                    vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                    vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                    vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
                ],
                false,
            ),
        ];

        for test_case in test_cases {
            let (board, expected) = dbg!(test_case);

            let result = Solution::is_valid_sudoku(board);

            assert_eq!(expected, result);
        }
    }
}
