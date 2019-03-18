pub struct Solution {}
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::cmp;
        use std::collections::HashSet;
        let nums: HashSet<_> = nums.into_iter().collect();
        let mut max = 0;
        for i in nums.iter() {
            if !nums.contains(&(i - 1)) {
                let mut len = 1;
                let mut j = i + 1;
                while nums.contains(&j) {
                    len += 1;
                    j += 1;
                }
                max = cmp::max(len, max);
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
        assert_eq!(4, Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
    }
}
