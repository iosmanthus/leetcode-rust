pub struct Solution {}
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut dp = vec![0; bytes.len()];
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
            }
        }

        dp.into_iter().max().unwrap_or(0) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        dbg!(Solution::longest_valid_parentheses("(()".to_owned()));
        dbg!(Solution::longest_valid_parentheses(")()())".to_owned()));
    }
}
