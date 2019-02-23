pub struct Solution {}
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }

        let mut i = nums.len() - 1;
        while i > 0 && nums[i] <= nums[i - 1] {
            i -= 1;
        }

        let rest = i;

        if i > 0 {
            let j = i - 1;
            while i < nums.len() && nums[i] > nums[j] {
                i += 1;
            }

            let tmp = nums[i - 1];
            nums[i - 1] = nums[j];
            nums[j] = tmp;
        }

        nums[rest..].reverse();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut vec = vec![
            vec![1, 2, 3],
            vec![1, 2, 5, 4, 6, 3],
            vec![3, 2, 1],
            vec![1, 3, 2],
        ];
        vec.iter_mut()
            .for_each(|vec| Solution::next_permutation(vec));

        assert_eq!(
            vec![
                vec![1, 3, 2],
                vec![1, 2, 5, 6, 3, 4],
                vec![1, 2, 3],
                vec![2, 1, 3]
            ],
            vec
        );
    }
}
