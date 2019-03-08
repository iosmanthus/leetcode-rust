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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn inorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            if let Some(node) = root {
                let mut left = inorder(&node.borrow().left);
                left.push(node.borrow().val);
                let mut right = inorder(&node.borrow().right);
                left.append(&mut right);
                left
            } else {
                vec![]
            }
        }
        inorder(&root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            vec![1, 0, 2, 1, 3],
            Solution::inorder_traversal(Some(Rc::new(RefCell::new(TreeNode {
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
    }
}
