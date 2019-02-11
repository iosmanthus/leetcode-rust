pub struct Solution {}
impl Solution {
    pub fn is_match(string: String, pattern: String) -> bool {
        let sbytes = string.as_bytes();
        let pbytes = pattern.as_bytes();

        if pattern.is_empty() {
            string.is_empty()
        } else {
            let first_match = !string.is_empty() && (sbytes[0] == pbytes[0] || pbytes[0] == b'.');
            if pattern.len() >= 2 && pbytes[1] == b'*' {
                (first_match
                    && Solution::is_match(string[1..].to_string(), pattern[..].to_string()))
                    || Solution::is_match(string[..].to_string(), pattern[2..].to_string())
            } else {
                first_match && Solution::is_match(string[1..].to_string(), pattern[1..].to_string())
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            false,
            Solution::is_match("ab".to_string(), ".*c".to_string())
        );
        assert_eq!(true, Solution::is_match("aa".to_string(), "a*".to_string()));

        assert_eq!(
            true,
            Solution::is_match("asdfjkl;".to_string(), ".*".to_string())
        );
    }
}
