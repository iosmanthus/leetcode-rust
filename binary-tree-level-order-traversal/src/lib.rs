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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut levels = vec![];
        let mut queue = VecDeque::new();

        if let Some(node) = root {
            levels.push(vec![node.borrow().val]);
            queue.push_back(node);
        }

        let mut pos = 0;
        while !queue.is_empty() {
            let mut level = vec![];
            for _ in levels[pos].iter() {
                let node = queue.pop_front().unwrap();
                if let Some(ref left) = node.clone().borrow().left {
                    level.push(left.borrow().val);
                    queue.push_back(left.clone());
                }
                if let Some(ref right) = node.clone().borrow().right {
                    level.push(right.borrow().val);
                    queue.push_back(right.clone());
                }
            }
            if !level.is_empty() {
                levels.push(level);
            }
            pos += 1;
        }
        levels
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            vec![vec![1], vec![2, 2], vec![3, 4, 4, 3]],
            Solution::level_order(Some(Rc::new(RefCell::new(TreeNode {
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
