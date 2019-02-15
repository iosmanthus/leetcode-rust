use std::cmp::Ordering;
pub fn binary_search_by<T, F>(slice: &[T], mut f: F) -> Result<usize, usize>
where
    F: FnMut(usize) -> Ordering,
{
    let (mut left, mut right) = (0, slice.len());
    while left < right {
        let mid = (left + right) >> 1;
        match f(mid) {
            Ordering::Greater => {
                right = mid;
            }
            Ordering::Less => {
                left = mid + 1;
            }
            Ordering::Equal => {
                return Ok(mid);
            }
        }
    }
    Err(left)
}

pub struct Solution {}
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (m, n) = (nums1.len(), nums2.len());

        let (a, b, m, n) = if m < n {
            (&nums1, &nums2, m, n)
        } else {
            (&nums2, &nums1, n, m)
        };
        let half = (m + n + 1) / 2;
        binary_search_by(a, |i| {
            let j = half - i - 1;
            if a[i] > b[j] {
                Ordering::Greater
            } else if i + 1 < m && j > 1 && a[i + 1] < b[j - 1] {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })
        .map(|i| i + 1)
        .or_else(|i| -> Result<usize, usize> { Ok(i) })
        .map(|i| {
            let j = half - i;
            let left_max = if i == 0 {
                b[j - 1]
            } else if j == 0 {
                a[i - 1]
            } else {
                std::cmp::max(a[i - 1], b[j - 1])
            };
            if (m + n) % 2 == 1 {
                left_max as f64
            } else {
                let right_min = if i == m {
                    b[j]
                } else if j == n {
                    a[i]
                } else {
                    std::cmp::min(a[i], b[j])
                };
                (left_max + right_min) as f64 / 2.0
            }
        })
        .unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            2.0,
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2])
        );
        assert_eq!(
            2.5,
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4])
        );
        assert_eq!(2.5, Solution::find_median_sorted_arrays(vec![], vec![2, 3]));
    }
}
