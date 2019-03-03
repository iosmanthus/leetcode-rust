pub struct Solution {}
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut paths = vec![1; n];
        for _ in 1..m {
            let mut prev = 0;
            for j in 0..n {
                paths[j] += prev;
                prev = paths[j];
            }
        }
        paths[n - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(3, Solution::unique_paths(3, 2));
        assert_eq!(28, Solution::unique_paths(7, 3));
    }
}
