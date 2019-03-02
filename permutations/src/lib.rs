pub struct Solution {}
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        fn permutations(candidates: HashSet<i32>) -> Vec<Vec<i32>> {
            if candidates.is_empty() {
                return vec![];
            }

            let mut result = vec![];
            for i in candidates.iter() {
                let mut candidates = candidates.clone();
                candidates.remove(i);
                let mut rest = permutations(candidates);
                if rest.is_empty() {
                    rest.push(vec![*i]);
                } else {
                    rest.iter_mut().for_each(|permutation| permutation.push(*i));
                }
                result.append(&mut rest);
            }
            result
        }
        permutations(nums.into_iter().collect())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        dbg!(Solution::permute(vec![1, 2, 3]));
    }
}
