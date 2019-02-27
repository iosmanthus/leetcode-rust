pub struct Solution {}
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp::Ordering;
        let l = nums
            .binary_search_by(|x| {
                if *x >= target {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            })
            .or_else(|x| -> Result<_, ()> { Ok(x) })
            .unwrap();
        let r = nums
            .binary_search_by(|x| {
                if *x <= target {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .or_else(|x| -> Result<_, ()> { Ok(x) })
            .unwrap();
        if l == r {
            vec![-1, -1]
        } else {
            vec![l as i32, r as i32 - 1]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            vec![2, 5],
            Solution::search_range(vec![1, 5, 6, 6, 6, 6, 7, 8, 8, 10, 11], 6)
        );
        assert_eq!(vec![-1, -1], Solution::search_range(vec![2, 2], 3));
    }
}
