use std::cell::RefCell;
use std::rc::Rc;
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
pub struct Solution {}
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn preorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
            let mut seq = vec![None];
            if let Some(node) = root {
                seq.push(Some(node.borrow().val));
                seq.append(&mut preorder(&node.borrow().left));
                seq.append(&mut preorder(&node.borrow().right));
            }
            seq
        }

        fn semipreoder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
            let mut seq = vec![None];
            if let Some(node) = root {
                seq.push(Some(node.borrow().val));
                seq.append(&mut semipreoder(&node.borrow().right));
                seq.append(&mut semipreoder(&node.borrow().left));
            }
            seq
        }

        if let Some(node) = root {
            preorder(&node.borrow().left) == semipreoder(&node.borrow().right)
        } else {
            true
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            true,
            Solution::is_symmetric(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                }))),
            }))))
        );
    }
}
