pub struct Solution {}
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut zero, mut one, mut two) = (0, 0, nums.len() - 1);
        while one <= two {
            if nums[one] == 0 {
                let temp = nums[one];
                nums[one] = nums[zero];
                nums[zero] = temp;

                zero += 1;
                one += 1;
            } else if nums[one] == 1 {
                one += 1;
            } else {
                let temp = nums[one];
                nums[one] = nums[two];
                nums[two] = temp;
                if two == 0 {
                    break;
                }
                two -= 1;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut vec = vec![2, 0, 1];
        Solution::sort_colors(&mut vec);
        assert_eq!(vec![0, 1, 2], vec);
    }
}
