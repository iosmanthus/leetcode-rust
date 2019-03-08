pub struct Solution {}
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut c = vec![1; n + 1];
        for i in 1..=n {
            c[i] = (0..i).map(|j| c[j] * c[i - j - 1]).sum();
        }
        c[n]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(5, Solution::num_trees(3));
    }
}
