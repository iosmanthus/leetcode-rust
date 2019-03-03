pub struct Solution {}
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp;
        let mut dp = grid;
        let (m, n) = (dp.len(), dp[0].len());
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if i == m - 1 && j == n - 1 {
                    continue;
                }
                dp[i][j] += if i == m - 1 {
                    dp[i][j + 1]
                } else if j == n - 1 {
                    dp[i + 1][j]
                } else {
                    cmp::min(dp[i][j + 1], dp[i + 1][j])
                };
            }
        }
        dp[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            7,
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]])
        );
    }
}
