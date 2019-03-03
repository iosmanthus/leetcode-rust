pub struct Solution {}
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut a, mut b) = (1, 1);
        for _ in 0..n {
            let next = a + b;
            a = b;
            b = next;
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(3, Solution::climb_stairs(3));
    }
}
