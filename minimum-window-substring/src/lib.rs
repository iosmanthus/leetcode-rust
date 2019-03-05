pub struct Solution {}
impl Solution {
    #[cfg(feature = "normal")]
    pub fn min_window(s: String, t: String) -> String {
        use std::collections::HashMap;

        let mut dict = HashMap::new();
        for c in t.bytes() {
            *dict.entry(c).or_insert(0) += 1;
        }

        let s = s.into_bytes();
        let require = dict.len();
        let mut formed = 0;
        let mut window = HashMap::new();
        let (mut l, mut r) = (0, 0);
        let mut min: Option<&[u8]> = None;

        while r < s.len() {
            let c = &s[r];
            if dict.contains_key(c) {
                *window.entry(*c).or_insert(0) += 1;
                if dict[c] == window[c] {
                    formed += 1;
                }
            }
            while formed == require {
                min = Some(if let Some(min) = min {
                    if r - l + 1 < min.len() {
                        &s[l..r + 1]
                    } else {
                        min
                    }
                } else {
                    &s[l..r + 1]
                });

                let c = &s[l];
                if dict.contains_key(c) {
                    *window.entry(*c).or_default() -= 1;
                    if window[c] < dict[c] {
                        formed -= 1;
                    }
                }
                l += 1;
            }
            r += 1;
        }
        String::from_utf8(Vec::from(min.unwrap_or("".as_bytes()))).unwrap()
    }

    #[cfg(feature = "optimized")]
    pub fn min_window(s: String, t: String) -> String {
        use std::collections::HashMap;
        let mut dict = HashMap::new();
        for c in t.bytes() {
            *dict.entry(c).or_insert(0) += 1;
        }

        let orgin = s.clone();
        let s: Vec<(_, _)> = s
            .bytes()
            .enumerate()
            .filter(|(_, c)| dict.contains_key(c))
            .collect();

        let require = dict.len();
        let mut formed = 0;

        let mut window = HashMap::new();
        let (mut l, mut r) = (0, 0);
        let mut min: Option<(usize, usize)> = None;

        while r < s.len() {
            let (end, ref c) = s[r];
            *window.entry(*c).or_insert(0) += 1;
            if dict[c] == window[c] {
                formed += 1;
            }
            while formed == require {
                let (begin, ref c) = s[l];
                min = Some(if let Some((i, j)) = min {
                    let min = j - i;
                    if end - begin < min {
                        (begin, end)
                    } else {
                        (i, j)
                    }
                } else {
                    (begin, end)
                });

                *window.entry(*c).or_default() -= 1;
                if window[c] < dict[c] {
                    formed -= 1;
                }

                l += 1;
            }
            r += 1;
        }

        if let Some((l, r)) = min {
            orgin[l..r + 1].to_string()
        } else {
            "".to_string()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            "aa".to_string(),
            Solution::min_window("aa".to_string(), "aa".to_string())
        );

        assert_eq!(
            "BANC".to_string(),
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string())
        );
    }
}
