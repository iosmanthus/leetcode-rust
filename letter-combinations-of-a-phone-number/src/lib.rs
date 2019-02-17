use std::collections::HashMap;
pub struct Solution {}
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let buttons: HashMap<char, &str> = [
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]
            .iter()
            .cloned()
            .collect();

        fn combi(digits: &str, buttons: &HashMap<char, &str>) -> Vec<String> {
            if let Some(digit) = digits.chars().next() {
                let candidates = buttons[&digit];
                let collections = combi(&digits[1..], buttons);

                candidates
                    .chars()
                    .map(|letter| {
                        if collections.is_empty() {
                            vec![letter.to_string()]
                        } else {
                            collections
                                .iter()
                                .map(|combi| letter.to_string() + combi)
                                .collect::<Vec<String>>()
                        }
                    })
                    .flatten()
                    .collect::<Vec<String>>()
            } else {
                vec![]
            }
        }

        combi(&digits, &buttons)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
                .iter()
                .map(|slice| slice.to_string())
                .collect::<Vec<String>>(),
            Solution::letter_combinations("23".to_string())
        );
    }
}
