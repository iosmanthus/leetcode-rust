pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        use std::collections::{HashMap, HashSet};
        let parentheses = ['[', '(', '{'].iter().cloned().collect::<HashSet<char>>();
        let pairs = [(']', '['), (')', '('), ('}', '{')]
            .iter()
            .cloned()
            .collect::<HashMap<char, char>>();

        let mut stack = vec![];
        for bracket in s.chars() {
            if parentheses.contains(&bracket) {
                stack.push(bracket);
            } else {
                if !stack.is_empty() && pairs.get(&bracket) == stack.last() {
                    stack.pop();
                } else {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(true, Solution::is_valid("([])".to_string()));
        assert_eq!(false, Solution::is_valid("([".to_string()));
        assert_eq!(false, Solution::is_valid("]".to_string()));
    }
}
