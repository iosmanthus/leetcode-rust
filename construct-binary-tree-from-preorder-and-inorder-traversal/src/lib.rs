// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = 0;
        fn build(
            preorder: &[i32],
            root_index: &mut usize,
            inorder: &[i32],
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if inorder.is_empty() {
                None
            } else {
                let mut root = TreeNode::new(preorder[*root_index]);
                let index = inorder
                    .iter()
                    .enumerate()
                    .find(|(_, &x)| x == root.val)
                    .unwrap()
                    .0;

                *root_index += 1;
                let left = build(preorder, root_index, &inorder[0..index]);
                let right = build(preorder, root_index, &inorder[index + 1..]);

                root.left = left;
                root.right = right;

                Some(Rc::new(RefCell::new(root)))
            }
        }
        build(&preorder, &mut root, &inorder)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        dbg!(Solution::build_tree(
            vec![3, 9, 20, 15, 7],
            vec![9, 3, 15, 20, 7]
        ));
    }
}
