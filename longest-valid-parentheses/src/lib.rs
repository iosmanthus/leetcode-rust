pub struct Solution {}
impl Solution {
    #[cfg(feature = "stack")]
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = vec![-1];
        let mut max = 0;
        for (i, ch) in s.chars().enumerate() {
            let i = i as i32;

            if ch == '(' {
                stack.push(i);
            }

            if ch == ')' {
                let _ = stack.pop().unwrap();
                if stack.is_empty() {
                    stack.push(i);
                } else {
                    max = std::cmp::max(i - stack.last().unwrap(), max);
                }
            }
        }
        max
    }

    #[cfg(feature = "dp")]
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut dp = vec![0; bytes.len()];
        let mut max = 0;
        for i in 0..dp.len() {
            dp[i] = if bytes[i] == b'(' {
                0
            } else if i > 0 && bytes[i] == b')' && bytes[i - 1] == b'(' {
                if i > 1 {
                    dp[i - 2] + 2
                } else {
                    2
                }
            } else if i > 0
                && bytes[i] == b')'
                && bytes[i - 1] == b')'
                && i > dp[i - 1]
                && bytes[i - dp[i - 1] - 1] == b'('
            {
                if i > dp[i - 1] + 1 {
                    dp[i - 1] + dp[i - dp[i - 1] - 2] + 2
                } else {
                    dp[i - 1] + 2
                }
            } else {
                0
            };
            max = std::cmp::max(max, dp[i]);
        }

        max as i32
    }

    #[cfg(feature = "best")]
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let (mut left, mut right) = (0, 0);
        let mut max = 0;
        for ch in s.chars() {
            if ch == '(' {
                left += 1;
            } else {
                right += 1;
            }
            if left == right {
                max = std::cmp::max(max, left * 2);
            } else if right > left {
                left = 0;
                right = 0;
            }
        }
        left = 0;
        right = 0;
        for ch in s.chars().rev() {
            if ch == '(' {
                left += 1;
            } else {
                right += 1;
            }
            if left == right {
                max = std::cmp::max(max, right * 2);
            } else if left > right {
                left = 0;
                right = 0;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2, Solution::longest_valid_parentheses("(()".to_owned()));
        assert_eq!(4, Solution::longest_valid_parentheses(")()())".to_owned()));
    }
}
