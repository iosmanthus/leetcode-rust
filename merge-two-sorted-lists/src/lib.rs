// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}
impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_some() && l2.is_some() {
            let (a, b) = (l1.as_ref().unwrap(), l2.as_ref().unwrap());
            let (val, next) = if a.val < b.val {
                (a.val, Solution::merge_two_lists(l1.unwrap().next, l2))
            } else {
                (b.val, Solution::merge_two_lists(l1, l2.unwrap().next))
            };
            Some(Box::new(ListNode { val, next }))
        } else if l1.is_none() {
            l2
        } else {
            l1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        dbg!(Solution::merge_two_lists(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 5, next: None }))
                }))
            })),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode { val: 8, next: None }))
                    }))
                }))
            })),
        ));
    }
}
