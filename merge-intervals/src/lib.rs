// Definition for an interval.
#[derive(Debug, PartialEq, Eq)]
pub struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    #[inline]
    pub fn new(start: i32, end: i32) -> Self {
        Interval { start, end }
    }
}

pub struct Solution {}
impl Clone for Interval {
    #[inline]
    fn clone(&self) -> Self {
        Interval::new(self.start, self.end)
    }
}
impl Solution {
    pub fn merge(intervals: Vec<Interval>) -> Vec<Interval> {
        if intervals.is_empty() {
            return vec![];
        }
        use std::cmp;
        let mut intervals = intervals;
        intervals.sort_by_key(|interval| interval.start);
        let mut collection = vec![];
        let mut iter = intervals[0].clone();
        for interval in intervals.iter().skip(1) {
            if iter.end >= interval.start {
                let end = cmp::max(iter.end, interval.end);
                iter = Interval::new(iter.start, end);
            } else {
                collection.push(iter);
                iter = interval.clone();
            }
        }
        collection.push(iter);
        collection
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            vec![
                Interval::new(1, 6),
                Interval::new(8, 10),
                Interval::new(15, 18),
            ],
            Solution::merge(vec![
                Interval::new(1, 3),
                Interval::new(15, 18),
                Interval::new(8, 10),
                Interval::new(2, 6),
            ])
        );
    }
}
