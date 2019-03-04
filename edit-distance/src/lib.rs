pub struct Solution {}
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (word1, word2) = (word1.as_bytes(), word2.as_bytes());
        let (m, n) = (word1.len(), word2.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 0..=m {
            for j in 0..=n {
                dp[i][j] = if i == 0 {
                    j
                } else if j == 0 {
                    i
                } else if word1[i - 1] == word2[j - 1] {
                    dp[i - 1][j - 1]
                } else {
                    *[dp[i - 1][j - 1], dp[i][j - 1], dp[i - 1][j]]
                        .into_iter()
                        .min()
                        .unwrap()
                        + 1
                }
            }
        }
        dp[m][n] as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            3,
            Solution::min_distance("horse".to_string(), "ros".to_string())
        );
    }
}
