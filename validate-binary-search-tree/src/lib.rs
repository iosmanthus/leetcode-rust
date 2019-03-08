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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn inorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut result = vec![];
            if let Some(node) = root {
                let mut left = inorder(&node.borrow().left);
                let mut right = inorder(&node.borrow().right);
                result.append(&mut left);
                result.push(node.borrow().val);
                result.append(&mut right);
            }
            result
        }
        let result = inorder(&root);
        for i in 1..result.len() {
            if result[i] <= result[i - 1] {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            true,
            Solution::is_valid_bst(Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
            }))))
        );
    }
}
