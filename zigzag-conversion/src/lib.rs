pub struct Solution {}
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let modulus = if num_rows == 1 { 1 } else { 2 * num_rows - 2 };
        let mut dict = vec![];
        dict.resize(num_rows as usize, vec![]);

        for (i, ch) in s.chars().enumerate() {
            let mut row = i as i32 % modulus;
            if row >= num_rows {
                row = modulus - row
            }
            dict[row as usize].push(ch);
        }
        dict.iter().flatten().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            "PAHNAPLSIIGYIR",
            Solution::convert("PAYPALISHIRING".to_string(), 3)
        );
        assert_eq!(
            "PINALSIGYAHRPI",
            Solution::convert("PAYPALISHIRING".to_string(), 4)
        );
    }
}
