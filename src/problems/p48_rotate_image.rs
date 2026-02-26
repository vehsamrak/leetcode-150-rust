pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut [Vec<i32>]) {
        Self::transpose(matrix);
        Self::reverse(matrix);
    }

    fn transpose(matrix: &mut [Vec<i32>]) {
        for i in 0..matrix.len() {
            for j in i..matrix.len() {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
    }

    fn reverse(matrix: &mut [Vec<i32>]) {
        let length = matrix.len();
        for i in 0..length {
            for j in 0..(length / 2) {
                matrix[i].swap(j, length - j - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let test_cases = [
            (
                vec![
                    vec![1, 2, 3], //
                    vec![4, 5, 6],
                    vec![7, 8, 9],
                ],
                vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]],
            ),
            (
                vec![
                    vec![5, 1, 9, 11],
                    vec![2, 4, 8, 10],
                    vec![13, 3, 6, 7],
                    vec![15, 14, 12, 16],
                ],
                vec![
                    vec![15, 13, 2, 5],
                    vec![14, 3, 4, 1],
                    vec![12, 6, 8, 9],
                    vec![16, 7, 10, 11],
                ],
            ),
        ];

        for test_case in test_cases {
            let (mut matrix, expected) = dbg!(test_case);

            Solution::rotate(&mut matrix);

            assert_eq!(expected, matrix);
        }
    }
}
