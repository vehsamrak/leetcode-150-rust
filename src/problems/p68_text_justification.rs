pub struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut result = Vec::new();
        let mut current_line: Vec<String> = Vec::new();
        let mut current_length = 0;

        for word in words {
            if current_length + word.len() + current_line.len() > max_width {
                let total_spaces = max_width - current_length;
                let line = if current_line.len() == 1 {
                    format!("{:<width$}", current_line[0], width = max_width)
                } else {
                    let gaps = current_line.len() - 1;
                    let (space_per_gap, extra) = (total_spaces / gaps, total_spaces % gaps);
                    current_line
                        .iter()
                        .enumerate()
                        .fold(String::new(), |mut s, (i, w)| {
                            s.push_str(w);
                            if i < gaps {
                                s.push_str(&" ".repeat(space_per_gap + (i < extra) as usize));
                            }
                            s
                        })
                };
                result.push(line);
                current_line.clear();
                current_length = 0;
            }

            current_length += word.len();
            current_line.push(word);
        }

        let last_line = current_line.join(" ");
        result.push(format!("{:<width$}", last_line, width = max_width));

        result
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
                    "This",
                    "is",
                    "an",
                    "example",
                    "of",
                    "text",
                    "justification.",
                ],
                16,
                vec!["This    is    an", "example  of text", "justification.  "],
            ),
            (
                vec!["What", "must", "be", "acknowledgment", "shall", "be"],
                16,
                vec!["What   must   be", "acknowledgment  ", "shall be        "],
            ),
            (
                vec![
                    "Science",
                    "is",
                    "what",
                    "we",
                    "understand",
                    "well",
                    "enough",
                    "to",
                    "explain",
                    "to",
                    "a",
                    "computer.",
                    "Art",
                    "is",
                    "everything",
                    "else",
                    "we",
                    "do",
                ],
                20,
                vec![
                    "Science  is  what we",
                    "understand      well",
                    "enough to explain to",
                    "a  computer.  Art is",
                    "everything  else  we",
                    "do                  ",
                ],
            ),
        ];

        for test_case in test_cases {
            let (words, max_width, expected) = dbg!(test_case);
            let words = words
                .into_iter()
                .map(String::from)
                .collect();

            let result = Solution::full_justify(words, max_width);

            assert_eq!(expected, result);
        }
    }
}
