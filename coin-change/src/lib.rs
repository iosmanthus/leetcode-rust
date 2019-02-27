pub struct Solution {}
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        use std::cmp;
        let mut dp = vec![0; amount as usize + 1];

        for i in 1..dp.len() {
            let mut min = None;
            for value in coins.iter() {
                let value = *value as usize;
                if i < value || dp[i - value] < 0 {
                    continue;
                }
                min = if min.is_none() {
                    Some(dp[i - value] + 1)
                } else {
                    Some(cmp::min(dp[i - value] + 1, min.unwrap()))
                }
            }
            dp[i] = if let Some(value) = min { value } else { -1 };
        }
        dp[amount as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(20, Solution::coin_change(vec![186, 419, 83, 408], 6249));
        assert_eq!(3, Solution::coin_change(vec![1, 2, 5], 11));
        assert_eq!(-1, Solution::coin_change(vec![2], 3));
    }
}
