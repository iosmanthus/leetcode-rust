pub struct Solution {}
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn subsets(nums: &[i32]) -> Vec<Vec<i32>> {
            if nums.is_empty() {
                vec![vec![]]
            } else {
                let mut exclude = subsets(&nums[1..]);
                let mut include = exclude
                    .clone()
                    .into_iter()
                    .map(|mut set| {
                        set.push(nums[0]);
                        set
                    })
                    .collect::<Vec<_>>();
                exclude.append(&mut include);
                exclude
            }
        }
        subsets(&nums)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(8, Solution::subsets(vec![1, 2, 3]).len());
    }
}
