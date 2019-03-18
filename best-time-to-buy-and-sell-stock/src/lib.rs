pub struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        use std::cmp;
        let mut max = 0;
        let (mut i, mut j) = (0, 0);
        while j < prices.len() {
            if prices[i] > prices[j] {
                i = j;
            } else {
                max = cmp::max(prices[j] - prices[i], max);
            }
            j += 1;
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(7, Solution::max_profit(vec![7, 1, 5, -1, 3, 6, 4]));
    }
}
