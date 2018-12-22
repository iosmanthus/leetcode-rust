use std::cmp;
use std::collections::HashSet;
pub struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut l, mut r, mut max) = (0, 0, 0);
        let mut set = HashSet::new();
        let bytes = s.as_bytes();
        while r < bytes.len() {
            if !set.insert(bytes[r]) {
                max = cmp::max(max, r - l);
                while bytes[l] != bytes[r] {
                    set.remove(&bytes[l]);
                    l += 1;
                }
                l += 1;
            }
            r += 1;
        }
        cmp::max(max, r - l) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(0, Solution::length_of_longest_substring("".to_string()));
        assert_eq!(1, Solution::length_of_longest_substring(" ".to_string()));
        assert_eq!(1, Solution::length_of_longest_substring("bbb".to_string()));
        assert_eq!(
            3,
            Solution::length_of_longest_substring("abcabcbb".to_string())
        );
        assert_eq!(
            3,
            Solution::length_of_longest_substring("pwwkew".to_string())
        );
        assert_eq!(
            5,
            Solution::length_of_longest_substring("adbcadcef".to_string())
        );
    }
}
