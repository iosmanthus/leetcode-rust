pub struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let mut max = 0;
        let mut longest = &s[0..0];
        for i in 0..s.len() {
            for j in i..i + 2 {
                let (mut begin, mut end) = (i, j);
                while end < s.len() {
                    if s[begin] == s[end] {
                        let len = end - begin + 1;
                        if len > max {
                            longest = &s[begin..end + 1];
                            max = len;
                        }
                    } else {
                        break;
                    }
                    if begin == 0 {
                        break;
                    }
                    begin -= 1;
                    end += 1;
                }
            }
        }
        String::from_utf8(longest.to_vec()).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            "bab".to_string(),
            Solution::longest_palindrome("babad".to_string())
        );
        assert_eq!(
            "bb".to_string(),
            Solution::longest_palindrome("cbbd".to_string())
        );
        assert_eq!(
            "cadddac".to_string(),
            Solution::longest_palindrome("cadddac".to_string())
        );
    }
}
