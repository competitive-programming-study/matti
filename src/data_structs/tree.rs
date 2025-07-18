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

impl<T: std::fmt::Display> TreeNode<T> {
    ///
    /// Base constructor
    ///
    pub fn new(val: T, sx: TreeNode<T>, dx: TreeNode<T>) -> Self {
        TreeNode {
            val,
            left: Some(Box::new(sx)),
            right: Some(Box::new(dx)),
        }
    }

    ///
    /// Tree with only left child constructor
    ///
    pub fn new_left(val: T, sub: TreeNode<T>) -> Self {
        TreeNode {
            val,
            left: Some(Box::new(sub)),
            right: None,
        }
    }

    ///
    /// Tree with only right child constructor
    ///
    pub fn new_right(val: T, sub: TreeNode<T>) -> Self {
        TreeNode {
            val,
            left: None,
            right: Some(Box::new(sub)),
        }
    }

    ///
    /// Leaf constructor
    ///
    pub fn new_leaf(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    ///
    /// Given a TreeNode returns an Option referencing the right child node
    ///
    pub fn get_left(&self) -> Option<&TreeNode<T>> {
        self.left.as_deref()
    }

    ///
    /// Given a TreeNode returns an Option referencing the right child node
    ///
    pub fn get_right(&self) -> Option<&TreeNode<T>> {
        self.right.as_deref()
    }

    ///
    /// Checks if a TreeNode is a leaf node
    ///
    pub fn is_leaf(&self) -> bool {
        match (self.get_left(), self.get_right()) {
            (None, None) => true,
            _ => false,
        }
    }

    ///
    /// String representation of a tree
    ///
    pub fn to_string(&self) -> String {
        fn build<T: std::fmt::Display>(
            node: &TreeNode<T>,
            prefix: String,
            is_left: bool,
            output: &mut String,
            has_sibling: bool,
        ) {
            if let Some(ref right) = node.right {
                let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
                build(right, new_prefix, false, output, node.left.is_some());
            }

            output.push_str(&prefix);
            if is_left {
                output.push_str("└── ");
            } else {
                output.push_str("┌── ");
            }
            output.push_str(&format!("{}\n", node.val));

            if let Some(ref left) = node.left {
                let new_prefix = format!(
                    "{}{}",
                    prefix,
                    if is_left && has_sibling {
                        "│   "
                    } else {
                        "    "
                    }
                );
                build(left, new_prefix, true, output, false);
            }
        }

        let mut result = String::new();
        build(self, "".to_string(), false, &mut result, false);
        result
    }
}
