pub struct Solution {}
impl Solution {
    pub fn is_match(text: String, pattern: String) -> bool {
        let mut matched = vec![vec![false; pattern.len() + 1]; 2];
        let (mut now, mut past) = (0, 1);
        for i in (0..text.len() + 1).rev() {
            for j in (0..pattern.len() + 1).rev() {
                let (text, pattern) = (&text[i..], &pattern[j..]);
                matched[now][j] = if pattern.is_empty() {
                    text.is_empty()
                } else {
                    let first_match = !text.is_empty()
                        && (text.as_bytes()[0] == pattern.as_bytes()[0]
                            || pattern.as_bytes()[0] == b'.');
                    if pattern.len() >= 2 && pattern.as_bytes()[1] == b'*' {
                        (first_match && matched[past][j]) || matched[now][j + 2]
                    } else {
                        first_match && matched[past][j + 1]
                    }
                };
            }
            std::mem::swap(&mut now, &mut past);
        }
        matched[past][0]
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
        assert_eq!(false, Solution::is_match("aa".to_string(), "a".to_string()));
        assert_eq!(
            true,
            Solution::is_match("aab".to_string(), "c*a*b".to_string())
        );
    }
}
