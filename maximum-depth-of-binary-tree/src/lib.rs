// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn max_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
            use std::cmp;
            if let Some(node) = root {
                cmp::max(
                    max_depth(&node.borrow().left),
                    max_depth(&node.borrow().right),
                ) + 1
            } else {
                0
            }
        }
        max_depth(&root) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            4,
            Solution::max_depth(Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: None,
                    })))
                }))),
            }))))
        );
        assert_eq!(0, Solution::max_depth(None));
    }
}
