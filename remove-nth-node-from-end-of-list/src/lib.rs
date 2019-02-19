// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}
impl Solution {
    #[cfg(feature = "safe")]
    pub fn remove_nth_from_end(list: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        pub fn remove(list: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
            match list {
                Some(node) => {
                    let (rest, tail_index) = remove(node.next, n);
                    let tail_index = tail_index + 1;
                    (
                        if tail_index == n {
                            rest
                        } else {
                            Some(Box::new(ListNode {
                                val: node.val,
                                next: rest,
                            }))
                        },
                        tail_index,
                    )
                }
                None => (None, 0),
            }
        }
        remove(list, n).0
    }

    #[cfg(feature = "unsafe")]
    pub fn remove_nth_from_end(list: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut list = Some(Box::new(ListNode { val: 0, next: list }));
        unsafe {
            let (mut scout, mut sapper): (*const _, *mut _) = (&list, &mut list);
            for _ in 0..=n {
                if let Some(node) = &*scout {
                    scout = &node.next;
                }
            }
            while let Some(node) = &*scout {
                if let Some(node) = &mut *sapper {
                    sapper = &mut node.next;
                }
                scout = &node.next;
            }

            if let Some(node) = &mut *sapper {
                let target = node.next.take();
                node.next = target.unwrap().next;
            }
        }
        list.unwrap().next
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode::new(5))),
                    })),
                })),
            })),
            Solution::remove_nth_from_end(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode::new(5))),
                            })),
                        })),
                    })),
                })),
                2,
            )
        );
        assert_eq!(
            None,
            Solution::remove_nth_from_end(Some(Box::new(ListNode { val: 1, next: None })), 1)
        );
    }
}
