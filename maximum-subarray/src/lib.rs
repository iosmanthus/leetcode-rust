pub struct Solution {}
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        use std::cmp;
        let mut prev = nums[0];
        let mut max = prev;
        for i in 1..nums.len() {
            prev = if prev < 0 { nums[i] } else { nums[i] + prev };
            max = cmp::max(prev, max);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            6,
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
        );
    }
}
