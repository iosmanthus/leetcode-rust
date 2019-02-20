use std::collections::HashSet;
pub struct Solution {}
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn solution(n: i32) -> HashSet<String> {
            if n == 0 {
                HashSet::new()
            } else {
                let prev = solution(n - 1);
                let prev = if prev.is_empty() {
                    [String::new()].iter().cloned().collect()
                } else {
                    prev
                };

                prev.into_iter()
                    .map(|combinations| {
                        (0..combinations.len() + 1)
                            .map(|index| {
                                format!("{}(){}", &combinations[..index], &combinations[index..])
                            })
                            .collect::<HashSet<_>>()
                    })
                    .flatten()
                    .collect::<HashSet<_>>()
            }
        }
        solution(n).into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            ["((()))", "(()())", "(())()", "()(())", "()()()"]
                .iter()
                .map(|slice| slice.to_string())
                .collect::<HashSet<String>>(),
            Solution::generate_parenthesis(3)
                .into_iter()
                .collect::<HashSet<_>>()
        );
    }
}
