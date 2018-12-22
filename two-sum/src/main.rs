use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();
        for (i, x) in nums.iter().enumerate() {
            if seen.contains_key(x) {
                return vec![seen[x], i as i32];
            } else {
                seen.insert(target - x, i as i32);
            }
        }
        unreachable!()
    }
}
fn main() {
    println!("{:?}", Solution::two_sum(vec![3, 2, 4, 8], 6));
}
