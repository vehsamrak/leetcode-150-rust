pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let publications_count = citations.len();
        let mut buckets = vec![0; publications_count + 1];

        for citations_count in citations {
            buckets[(citations_count as usize).min(publications_count)] += 1;
        }

        let mut count = 0;
        for (i, &papers_in_bucket) in buckets
            .iter()
            .enumerate()
            .rev()
        {
            count += papers_in_bucket;
            if count >= i {
                return i as i32;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [(vec![3, 0, 6, 1, 5], 3), (vec![1, 3, 1], 1)];

        for test_case in test_cases {
            let (input, expected) = test_case;

            let result = Solution::h_index(input);

            assert_eq!(expected, result);
        }
    }
}
