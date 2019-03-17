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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn flatten(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = root {
                let l = flatten(node.borrow().left.clone());
                let r = flatten(node.borrow().right.clone());
                if l.is_none() && r.is_none() {
                    return Some(node.clone());
                }
                if let Some(ref l) = l {
                    l.borrow_mut().right = node.borrow_mut().right.take();
                    let mut node = node.borrow_mut();
                    node.right = node.left.take();
                }
                r.or(l)
            } else {
                None
            }
        }
        flatten(root.clone());
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        Solution::flatten(&mut root);
        dbg!(root);
    }
}
