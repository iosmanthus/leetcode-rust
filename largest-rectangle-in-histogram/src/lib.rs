pub struct Solution {}
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        use std::cmp;

        let mut stack = vec![];

        let mut i = 0;
        let mut max = 0;
        while i < heights.len() {
            if stack.is_empty() || heights[i] >= heights[*stack.last().unwrap()] {
                stack.push(i);
                i += 1;
            } else {
                let top = *stack.last().unwrap();
                let _ = stack.pop().unwrap();
                max = cmp::max(
                    max,
                    heights[top] * if stack.is_empty() {
                        i
                    } else {
                        i - *stack.last().unwrap() - 1
                    } as i32,
                )
            }
        }
        while !stack.is_empty() {
            let top = *stack.last().unwrap();
            let _ = stack.pop().unwrap();
            max = cmp::max(
                max,
                heights[top] * if stack.is_empty() {
                    i
                } else {
                    i - *stack.last().unwrap() - 1
                } as i32,
            )
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(10, Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
    }
}
