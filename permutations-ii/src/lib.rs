pub struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::solve(nums.into_iter().enumerate().collect())
    }
    fn solve(nums: HashMap<usize, i32>) -> Vec<Vec<i32>> {
        if nums.len() <= 1 {
            return vec![nums.values().copied().collect()];
        }

        let mut unique = HashSet::new();
        let mut solutions = vec![];
        for (&index, &v) in nums.iter() {
            if !unique.insert(v) {
                continue;
            }

            solutions.append(
                &mut Self::solve(
                    nums.iter()
                        .filter(|(&k, _)| k != index)
                        .map(|(k, v)| (*k, *v))
                        .collect(),
                )
                .into_iter()
                .map(|mut seq| {
                    seq.push(v);
                    seq
                })
                .collect(),
            );
        }
        solutions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let cases = vec![(
            vec![1, 1, 2],
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]],
        )];
        for (input, expected) in cases {
            let actual = Solution::permute_unique(input);
            // `Expected' contains `Actual'
            for seq in actual.iter() {
                assert!(expected.iter().find(|s| s == &seq).is_some());
            }

            // `Actual' contains `Expected'
            for seq in expected.iter() {
                assert!(actual.iter().find(|s| s == &seq).is_some());
            }
        }
    }
}
