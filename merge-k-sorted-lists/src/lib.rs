// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

impl ListNode {
    pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut new = ListNode::to_node(0);
        let mut tail = &mut new;
        for x in vec {
            if let Some(node) = tail {
                node.next = ListNode::to_node(x);
                tail = &mut node.next;
            }
        }
        new.unwrap().next
    }
}

use std::cmp::{Ord, Ordering, PartialOrd};
impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl ListNode {
    #[inline]
    pub fn to_node(val: i32) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode { val, next: None }))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();

        // Initialize
        for list in lists {
            if list.is_some() {
                heap.push(Reverse(list));
            }
        }

        let mut new = ListNode::to_node(0);
        let mut tail_ptr = &mut new;
        while !heap.is_empty() {
            let Reverse(mut min) = heap.pop().unwrap();
            if let Some(node) = &mut min {
                if node.next.is_some() {
                    heap.push(Reverse(node.next.take()));
                }
            }
            if let Some(node) = tail_ptr {
                node.next = min;
                tail_ptr = &mut node.next;
            }
        }

        new.unwrap().next
    }
}

pub struct Solution {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            ListNode::from_vec(vec![1, 1, 2, 3, 4, 4, 5, 6]),
            Solution::merge_k_lists(vec![
                ListNode::from_vec(vec![1, 4, 5]),
                ListNode::from_vec(vec![1, 3, 4]),
                ListNode::from_vec(vec![2, 6]),
            ])
        );
    }
}
