use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<String, Vec<String>> = HashMap::with_capacity(strs.len());
        for word in strs {
            let mut sorted_word: Vec<char> = word.chars().collect();
            sorted_word.sort_unstable();
            let sorted_word: String = sorted_word
                .into_iter()
                .collect();

            groups
                .entry(sorted_word)
                .or_default()
                .push(word);
        }

        groups.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            (
                vec!["eat", "tea", "tan", "ate", "nat", "bat"],
                vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]],
            ),
            (vec![""], vec![vec![""]]),
            (vec!["a"], vec![vec!["a"]]),
        ];

        for test_case in test_cases {
            let (strs, expected) = dbg!(test_case);

            let mut result = Solution::group_anagrams(
                strs.into_iter()
                    .map(String::from)
                    .collect(),
            );

            // Sort inner and outer vectors for comparison
            for inner in &mut result {
                inner.sort();
            }
            result.sort();

            let mut expected_sorted: Vec<Vec<String>> = expected
                .into_iter()
                .map(|v| {
                    v.into_iter()
                        .map(String::from)
                        .collect()
                })
                .collect();
            for inner in &mut expected_sorted {
                inner.sort();
            }
            expected_sorted.sort();

            assert_eq!(expected_sorted, result);
        }
    }
}
