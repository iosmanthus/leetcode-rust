pub struct Solution {}
impl Solution {
    fn find_pivot(nums: &[i32]) -> usize {
        assert!(!nums.is_empty());
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let mid = (l + r) / 2;
            if nums[l] > nums[mid] {
                r = mid;
            } else if nums[mid] > nums[r] {
                l = mid + 1;
            } else {
                break;
            }
        }
        l
    }
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let pivot = Self::find_pivot(&nums);
        println!("{}", pivot);
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let mid = (l + r) / 2;
            let mapping = (mid + pivot) % nums.len();
            let value = nums[mapping];
            if target == value {
                return mapping as i32;
            } else if target < value {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        let mapping = (l + pivot) % nums.len();
        if nums[mapping] == target {
            mapping as i32
        } else {
            -1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        dbg!(Solution::search(vec![8, 9, 2, 3, 4], 9));
    }
}
