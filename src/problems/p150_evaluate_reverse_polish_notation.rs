pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        tokens
            .iter()
            .try_fold(
                Vec::with_capacity(tokens.len()),
                |mut stack: Vec<i32>, token| {
                    if let Ok(number) = token.parse::<i32>() {
                        stack.push(number);
                    } else {
                        let (first, second) = (stack.pop()?, stack.pop()?);
                        let result = match token.as_str() {
                            "+" => second + first,
                            "-" => second - first,
                            "*" => second * first,
                            "/" => second / first,
                            _ => return None,
                        };

                        stack.push(result);
                    }

                    Some(stack)
                },
            )
            .and_then(|mut stack| stack.pop())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            (vec!["2", "1", "+", "3", "*"], 9),
            (vec!["4", "13", "5", "/", "+"], 6),
            (
                vec![
                    "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
                ],
                22,
            ),
        ];

        for test_case in test_cases {
            let (tokens, expected) = dbg!(test_case);
            let tokens = tokens
                .into_iter()
                .map(|s| s.to_string())
                .collect();

            let result = Solution::eval_rpn(tokens);

            assert_eq!(expected, result);
        }
    }
}
