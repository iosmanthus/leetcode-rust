pub struct Solution {}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, x| acc ^ x)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(1, Solution::single_number(vec![2, 2, 1]));
        assert_eq!(4, Solution::single_number(vec![4, 1, 2, 1, 2]));
    }
}
