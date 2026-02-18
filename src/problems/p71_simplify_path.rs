pub struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        format!(
            "/{}",
            path.split('/')
                .fold(Vec::new(), |mut stack, part| {
                    match part {
                        "" | "." => {}
                        ".." => {
                            stack.pop();
                        }
                        _ => stack.push(part),
                    }
                    stack
                })
                .join("/")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            ("/home/", "/home"),
            ("/../", "/"),
            ("/home//foo/", "/home/foo"),
            ("/a/./b/../../c/", "/c"),
            ("/.../a/../b/c/../d/./", "/.../b/d"),
        ];

        for test_case in test_cases {
            let (path, expected) = dbg!(test_case);

            let result = Solution::simplify_path(path.to_string());

            assert_eq!(expected, result);
        }
    }
}
