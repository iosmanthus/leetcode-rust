pub struct Solution {}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        fn solution(
            s: &str,
            dict: &[String],
            offset: usize,
            marker: &mut Vec<Option<bool>>,
        ) -> bool {
            if s.is_empty() {
                return true;
            }
            if let Some(matched) = marker[offset] {
                return matched;
            }
            for word in dict {
                if s.starts_with(word)
                    && solution(&s[word.len()..], dict, offset + word.len(), marker)
                {
                    marker[offset] = Some(true);
                    return true;
                }
            }
            marker[offset] = Some(false);
            false
        }
        solution(&s, &word_dict, 0, &mut vec![None; s.len() + 1])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            false,
            Solution::word_break(
                "catsandog".to_string(),
                ["cats", "dog", "sand", "and", "cat"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect()
            )
        );
        assert_eq!(
            true,
            Solution::word_break(
                "leetcode".to_string(),
                ["leet", "code"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect()
            )
        );
    }
}
