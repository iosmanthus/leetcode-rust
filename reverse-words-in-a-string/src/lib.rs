pub struct Solution {}
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            "blue is sky the".to_string(),
            Solution::reverse_words("the sky is blue".to_string())
        );
    }
}
