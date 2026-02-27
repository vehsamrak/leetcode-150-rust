pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (a, b) = (a.as_bytes(), b.as_bytes());

        let mut result = String::with_capacity(a.len().max(b.len()) + 1);
        let (mut i, mut j) = (a.len(), b.len());
        let mut carry = 0;
        while i > 0 || j > 0 || carry != 0 {
            let mut sum = carry;

            if i > 0 {
                sum += a[i - 1] - b'0';
                i -= 1;
            }

            if j > 0 {
                sum += b[j - 1] - b'0';
                j -= 1;
            }

            match sum {
                0 | 2 => result.push('0'),
                1 | 3 => result.push('1'),
                _ => unreachable!(),
            }

            carry = if sum >= 2 { 1 } else { 0 };
        }

        result.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            ("11", "1", "100"), //
            ("1010", "1011", "10101"),
        ];

        for test_case in test_cases {
            let (a, b, expected) = dbg!(test_case);

            let result = Solution::add_binary(a.to_string(), b.to_string());

            assert_eq!(expected.to_string(), result);
        }
    }
}
