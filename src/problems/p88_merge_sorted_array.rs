pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
        let (mut m, mut n) = (m as usize, n as usize);

        while n != 0 {
            if m == 0 || nums1[m - 1] < nums2[n - 1] {
                nums1[m + n - 1] = nums2[n - 1];
                n -= 1;
            } else {
                nums1[m + n - 1] = nums1[m - 1];
                m -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            (
                vec![1, 2, 3, 0, 0, 0],
                3,
                vec![2, 5, 6],
                3,
                vec![1, 2, 2, 3, 5, 6],
            ),
            (vec![1], 1, vec![], 0, vec![1]),
            (vec![0], 0, vec![1], 1, vec![1]),
            (
                vec![4, 0, 0, 0, 0, 0],
                1,
                vec![1, 2, 3, 5, 6],
                5,
                vec![1, 2, 3, 4, 5, 6],
            ),
        ];

        for test_case in test_cases {
            let (mut nums1, m, mut nums2, n, expected) = test_case;

            Solution::merge(&mut nums1, m, &mut nums2, n);

            assert_eq!(expected, nums1);
        }
    }
}
