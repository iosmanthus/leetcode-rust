pub struct Solution {}
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut i = 0;

        // Move to the first-nowhitespace char
        while i < bytes.len() && bytes[i] == b' ' {
            i += 1;
        }

        if i == bytes.len() {
            return 0;
        }

        // Check the sign
        let signed = if bytes[i] == b'-' {
            i += 1;
            true
        } else if bytes[i] == b'+' {
            i += 1;
            false
        } else if bytes[i].is_ascii_digit() {
            false
        } else {
            return 0;
        };

        let mut value = 0;

        while i < bytes.len() && bytes[i].is_ascii_digit() {
            let digit = (bytes[i] - b'0') as i64;
            value = digit + value * 10;

            // Check if there is an overflow
            let max = std::i32::MAX as i64 + signed as i64;
            if value > max {
                value = max;
                break;
            }
            i += 1;
        }

        if signed {
            -value as i32
        } else {
            value as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(-4193, Solution::my_atoi("    -4193".to_string()));
        assert_eq!(4193, Solution::my_atoi("4193 add 1".to_string()));
        assert_eq!(0, Solution::my_atoi("input 4193 add 1".to_string()));
        assert_eq!(4193, Solution::my_atoi("+4193".to_string()));
        assert_eq!(-4193, Solution::my_atoi("-4193".to_string()));
        assert_eq!(
            std::i32::MIN,
            Solution::my_atoi("-41938490358052839023".to_string())
        );
        assert_eq!(
            std::i32::MAX,
            Solution::my_atoi("412431938490358052839023".to_string())
        );
    }
}
