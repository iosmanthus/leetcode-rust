pub struct Solution {}
impl Solution {
    #[cfg(feature = "dp")]
    pub fn trap(height: Vec<i32>) -> i32 {
        use std::cmp;

        if height.is_empty() {
            return 0;
        }

        let (mut max_left, mut max_right) = (vec![0; height.len()], vec![0; height.len()]);

        for i in 1..max_left.len() {
            max_left[i] = cmp::max(max_left[i - 1], height[i - 1]);
        }
        for i in (0..max_left.len() - 1).rev() {
            max_right[i] = cmp::max(max_right[i + 1], height[i + 1]);
        }

        height
            .into_iter()
            .enumerate()
            .fold(0, |total, (i, height)| {
                let shorter = cmp::min(max_left[i], max_right[i]);
                total + if shorter <= height {
                    0
                } else {
                    shorter - height
                }
            })
    }

    #[cfg(feature = "two_pointers")]
    pub fn trap(height: Vec<i32>) -> i32 {
        use std::cmp;

        if height.len() < 3 {
            return 0;
        }
        let (mut left, mut right) = (1, height.len() - 2);
        let (mut max_left, mut max_right) = (0, 0);
        let mut sum = 0;
        for _ in 1..height.len() - 1 {
            if height[left - 1] < height[right + 1] {
                max_left = cmp::max(max_left, height[left - 1]);
                if max_left > height[left] {
                    sum += max_left - height[left];
                }
                left += 1;
            } else {
                max_right = cmp::max(max_right, height[right + 1]);
                if max_right > height[right] {
                    sum += max_right - height[right];
                }
                right -= 1;
            }
        }
        sum
    }

    #[cfg(feature = "stack")]
    pub fn trap(height: Vec<i32>) -> i32 {
        use std::cmp;
        let mut stack = vec![];
        let mut total = 0;

        for i in 0..height.len() {
            while !stack.is_empty() && height[i] > height[*stack.last().unwrap()] {
                let slot = *stack.last().unwrap();
                stack.pop();
                if stack.is_empty() {
                    break;
                }
                let w = (i - *stack.last().unwrap() - 1) as i32;
                let h = cmp::min(height[i], height[*stack.last().unwrap()]) - height[slot];
                total += w * h;
            }
            stack.push(i);
        }
        total
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(6, Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
    }
}
