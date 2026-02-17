use std::collections::HashSet;

pub struct Solution;

impl Solution {
    // Floyd circle detection solution
    pub fn is_happy(number: i32) -> bool {
        if number < 0 {
            return false;
        }

        let check_next = |mut n: i32| {
            let mut sum = 0;
            while n > 0 {
                sum += (n % 10).pow(2);
                n /= 10;
            }

            sum
        };

        let mut slow = check_next(number);
        let mut fast = check_next(slow);

        while fast != 1 && slow != fast {
            slow = check_next(slow);
            fast = check_next(check_next(fast));
        }

        fast == 1
    }

    // solution with hash set
    pub fn is_happy_hashset(number: i32) -> bool {
        let mut seen: HashSet<i32> = HashSet::new();
        let mut n = number;

        while n != 1 {
            if seen.contains(&n) {
                return false;
            }

            seen.insert(n);

            let mut sum = 0;
            while n > 0 {
                sum += (n % 10).pow(2);
                n /= 10;
            }

            n = sum;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [(19, true), (2, false)];

        for test_case in test_cases {
            let (number, expected) = dbg!(test_case);

            let result = Solution::is_happy(number);
            assert_eq!(expected, result, "Failed for n: {}", number);
            let result = Solution::is_happy_hashset(number);
            assert_eq!(expected, result, "Failed for n: {}", number);
        }
    }
}
