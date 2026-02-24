pub struct Solution;

impl Solution {
    // internal rust function
    pub fn reverse_bits(mut n: i32) -> i32 {
        for i in 0..16 {
            let left_bit = (n >> i) & 1;
            let right_bit = (n >> (31 - i)) & 1;

            if left_bit != right_bit {
                n ^= 1 << i;
                n ^= 1 << (31 - i);
            }
        }

        n
    }

    // internal rust function
    pub fn reverse_bits_internal(n: i32) -> i32 {
        n.reverse_bits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [(43261596, 964176192), (2147483644, 1073741822)];

        for test_case in test_cases {
            let (input, expected) = dbg!(test_case);

            let result = Solution::reverse_bits(input);
            assert_eq!(expected, result);
            let result = Solution::reverse_bits_internal(input);
            assert_eq!(expected, result);
        }
    }
}
