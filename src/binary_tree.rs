use std::fmt::Debug;

#[derive(Debug, Clone, Default)]
pub struct TreeNode<T: Ord> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>
}

impl<T: Ord + Default> TreeNode<T> {
    pub fn new() -> Self {
        TreeNode {
            value: T::default(),
            left: None,
            right: None,
        }
    }

    pub fn from(value: T) -> Self
    {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: T) {
        if value <= self.value {
            if let Some(left) = &mut self.left {
                left.insert(value)
            } else {
                self.left = Some(Box::new(TreeNode::from(value)))
            }
        }
        else {
            if let Some(right) = &mut self.right {
                right.insert(value)
            } else {
                self.right = Some(Box::new(TreeNode::from(value)))
            }
        }
    }

    pub fn search(&self, value:T) -> bool {
        if self.value == value {
            true
        } else if value <= self.value {
            self.left.as_ref().map_or(false, |left| {left.search(value)})
        } else {
            self.right.as_ref().map_or(false, |right| {right.search(value)})
        }
    }
}