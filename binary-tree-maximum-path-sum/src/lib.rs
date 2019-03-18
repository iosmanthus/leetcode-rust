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
pub struct Solution;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::cmp;
        fn traverse(root: Option<Rc<RefCell<TreeNode>>>, max: &mut Option<i32>) -> i32 {
            if let Some(node) = root {
                let l = traverse(node.borrow().left.clone(), max);
                let r = traverse(node.borrow().right.clone(), max);
                let val = node.borrow().val;
                let branch = cmp::max(val, val + cmp::max(l, r));
                *max = cmp::max(Some(l + r + val), *max);
                *max = cmp::max(Some(branch), *max);
                branch
            } else {
                0
            }
        }
        let mut max = None;
        traverse(root, &mut max);
        max.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: -10,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));
        assert_eq!(42, Solution::max_path_sum(root));
    }
}
