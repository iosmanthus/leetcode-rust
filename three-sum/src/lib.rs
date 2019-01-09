use std::collections::HashSet;
pub struct Solution {}
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        nums.sort();
        let mut solution = HashSet::new();
        for (i, x) in nums.iter().enumerate() {
            if *x > 0 {
                break;
            }
            let sum = -x;
            let mut seen = HashSet::new();
            for y in nums[i + 1..].iter() {
                if seen.contains(y) {
                    solution.insert(vec![*x, sum - y, *y]);
                } else {
                    seen.insert(sum - y);
                }
            }
        }
        solution.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            vec![vec![-1, 0, 1], vec![-1, -1, 2]],
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
        );
    }
}
