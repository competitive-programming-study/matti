#[macro_export]
macro_rules! tree {
    ($val:expr, $left:expr, $right:expr) => {
        TreeNode::new($val, $left, $right)
    };
}

#[macro_export]
macro_rules! tree_left {
    ($val:expr, $left:expr) => {
        TreeNode::new_left($val, $left)
    };
}

#[macro_export]
macro_rules! tree_right {
    ($val:expr, $right:expr) => {
        TreeNode::new_right($val, $right)
    };
}

#[macro_export]
macro_rules! tree_leaf {
    ($val:expr) => {
        TreeNode::new_leaf($val)
    };
}

#[derive(Debug)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn new(val: T, sx: TreeNode<T>, dx: TreeNode<T>) -> Self {
        TreeNode {
            val,
            left: Some(Box::new(sx)),
            right: Some(Box::new(dx)),
        }
    }

    pub fn new_left(val: T, sub: TreeNode<T>) -> Self {
        TreeNode {
            val,
            left: Some(Box::new(sub)),
            right: None,
        }
    }

    pub fn new_right(val: T, sub: TreeNode<T>) -> Self {
        TreeNode {
            val,
            left: None,
            right: Some(Box::new(sub)),
        }
    }

    pub fn new_leaf(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    // Returns an Option with a reference to the left child node
    pub fn left(&self) -> Option<&TreeNode<T>> {
        self.left.as_deref()
    }

    // Returns an Option with a reference to the right child node
    pub fn right(&self) -> Option<&TreeNode<T>> {
        self.right.as_deref()
    }
}

pub fn max_path(bin: &TreeNode<i32>) -> i32 {
    //Tuple having (max_path in subtree , sum_special_nodes in subtree)
    fn max_path_rec(bin: &TreeNode<i32>) -> (i32, i32) {
        match (bin.left(), bin.right()) {
            (None, None) => (bin.val, i32::MIN),

            (Some(t), None) | (None, Some(t)) => {
                let (max_path, max_sum) = max_path_rec(t);
                (bin.val + max_path, max_sum)
            }

            (Some(s), Some(d)) => {
                let (sx_path, sx_sum) = max_path_rec(s);
                let (dx_path, dx_sum) = max_path_rec(d);

                let max_path = bin.val + sx_path.max(dx_path);
                let sum = sx_path + dx_path + bin.val;

                (max_path, sum.max(sx_sum).max(dx_sum))
            }
        }
    }

    max_path_rec(bin).1
}
