pub struct Solution {}
impl Solution {
    fn hash(string: &str) -> [usize; 26] {
        let mut alphabet = [0; 26];
        string.chars().for_each(|ch| {
            alphabet[ch as usize - 'a' as usize] += 1;
        });
        alphabet
    }
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut mapping = HashMap::new();
        strs.into_iter().for_each(|string| {
            mapping
                .entry(Self::hash(&string))
                .or_insert_with(|| vec![])
                .push(string);
        });
        mapping.values().cloned().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        dbg!(Solution::group_anagrams(
            ["eat", "tea", "tan", "ate", "nat", "bat"]
                .into_iter()
                .map(String::from)
                .collect()
        ));
    }
}
