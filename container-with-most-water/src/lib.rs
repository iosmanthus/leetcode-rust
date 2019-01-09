pub struct Solution {}
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp;
        let succ = |x: &mut usize| -> usize {
            let t = *x;
            *x += 1;
            t
        };
        let pred = |x: &mut usize| -> usize {
            let t = *x;
            *x -= 1;
            t
        };

        let (mut l, mut r) = (0, height.len() - 1);
        let mut max = 0;
        while l != r {
            let w = (r - l) as i32;
            let h = if height[l] > height[r] {
                height[pred(&mut r)]
            } else {
                height[succ(&mut l)]
            };
            max = cmp::max(max, h * w);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }
}
