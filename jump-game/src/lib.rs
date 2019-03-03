pub struct Solution {}
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut left_most = nums.len() - 1;
        for i in (0..nums.len() - 1).rev() {
            if i + nums[i] as usize >= left_most {
                left_most = i;
            }
        }
        left_most == 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        dbg!(Solution::can_jump(vec![3, 2, 1, 1, 4]));
    }
}
