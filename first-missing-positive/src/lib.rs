use std::mem;

pub struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            while nums[i] > 0 && (nums[i] as usize) < nums.len() && nums[i] as usize != i + 1 {
                let target = nums[i];
                if nums[target as usize - 1] == nums[i] {
                    break;
                }
                nums[i] = mem::replace(&mut nums[target as usize - 1], target);
            }
        }

        (nums
            .iter()
            .enumerate()
            .find(|&(i, x)| i + 1 != *x as usize)
            .map(|(i, _)| i)
            .unwrap_or_else(|| nums.len())
            + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let cases = vec![
            (vec![1, 2, 3, 2], 4),
            (vec![1, 2, 0], 3),
            (vec![2, 1], 3),
            (vec![9, 8, 7, 6, 5, 2, 4, 2, 1], 3),
            (vec![8, 9, 7, 6, 5, 4, 3, 2, 1], 10),
            (vec![4, 3, 6, 2, 1], 5),
            (vec![4, 6, 3, 1, 5, 2], 7),
            (vec![3, 4, -1, 1], 2),
            (vec![7, 8, 9, 11, 12], 1),
        ];

        for (input, expected) in cases {
            assert_eq!(expected, Solution::first_missing_positive(input));
        }
    }
}
