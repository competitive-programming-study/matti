use std::{cmp::Ord, iter::Sum, ops::Add};

pub struct Node<T> {
    pub key: T,
    pub id_left: Option<usize>,
    pub id_right: Option<usize>,
}

impl<T> Node<T> {
    /// Creates a new `Node` with the given key and no children.
    ///
    /// # Examples
    /// ```
    /// struct Node<T> {
    /// #     pub key: T,
    /// #     pub id_left: Option<usize>,
    /// #     pub id_right: Option<usize>,
    /// # }
    /// # impl<T> Node<T> {
    /// #     fn new(key: T) -> Self {
    /// #         Self {key,id_left: None,id_right: None,}
    /// #     }
    /// # }
    /// let node = Node::new(10);
    /// assert_eq!(node.key, 10);
    /// assert_eq!(node.id_left, None);
    /// assert_eq!(node.id_right, None);
    ///
    /// let string_node = Node::new("hello");
    /// assert_eq!(string_node.key, "hello");
    /// ```
    pub fn new(key: T) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Tree<T> {
    pub nodes: Vec<Node<T>>,
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Tree<T> {
    /// Creates a new, empty `Tree`.
    ///
    /// # Examples
    ///
    /// ```
    /// # struct Node<T> {
    /// #     pub key: T,
    /// #     pub id_left: Option<usize>,
    /// #     pub id_right: Option<usize>,
    /// # }
    /// # impl<T> Node<T> {
    /// #     fn new(key: T) -> Self {
    /// #         Self {key,id_left: None,id_right: None,}
    /// #     }
    /// # }
    /// # struct Tree<T> {
    /// #     nodes: Vec<Node<T>>,
    /// # }
    /// # impl<T> Tree<T> {
    /// #     pub fn new() -> Self {
    /// #         Self { nodes: vec![] }
    /// #     }
    /// # }
    /// let tree: Tree<i32> = Tree::new();
    /// assert!(tree.nodes.is_empty());
    /// ```
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    /// Creates a new `Tree` with a single root node containing the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// # struct Node<T> {
    /// #     pub key: T,
    /// #     pub id_left: Option<usize>,
    /// #     pub id_right: Option<usize>,
    /// # }
    /// # impl<T> Node<T> {
    /// #     fn new(key: T) -> Self {
    /// #         Self {key,id_left: None,id_right: None,}
    /// #     }
    /// # }
    /// # struct Tree<T> {
    /// #     nodes: Vec<Node<T>>,
    /// # }
    /// # impl<T> Tree<T> {
    /// #     pub fn with_root(key: T) -> Self {
    /// #         Self { nodes: vec![Node::new(key)]}
    /// #     }
    /// # }
    /// let tree = Tree::with_root(42);
    /// assert_eq!(tree.nodes.len(), 1);
    /// assert_eq!(tree.nodes[0].key, 42);
    /// ```
    pub fn with_root(key: T) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a new node to the binary tree as a child of the specified parent.
    ///
    /// # Arguments
    ///
    /// * `parent_id` - The index of the parent node in the `nodes` vector.
    /// * `key` - The value to store in the new node.
    /// * `is_left` - If `true`, adds as the left child; if `false`, adds as the right child.
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - The index of the newly added node.
    /// * `Err(&str)` - An error if the parent doesn't exist or already has the specified child.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The parent ID is invalid.
    /// - The specified child (left or right) already exists.
    pub fn add_node(&mut self, parent_id: usize, key: T, is_left: bool) -> Result<usize, &str> {
        if parent_id >= self.nodes.len() {
            return Err("Parent node id does not exist");
        }

        if is_left && self.nodes[parent_id].id_left.is_some() {
            return Err("Parent node has the left child already set");
        }

        if !is_left && self.nodes[parent_id].id_right.is_some() {
            return Err("Parent node has the left child already set");
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        Ok(child_id)
    }

    /// Adds a new node as the **left child** of the specified parent node.
    ///
    /// This is a convenience wrapper around [`add_node`] with `is_left` set to `true`.
    ///
    /// # Arguments
    ///
    /// * `parent_id` - The index of the parent node.
    /// * `key` - The value to store in the new node.
    ///
    /// # Returns
    ///
    /// Returns the index of the newly added node or an error if adding fails.
    pub fn add_node_left(&mut self, parent_id: usize, key: T) -> Result<usize, &str> {
        self.add_node(parent_id, key, true)
    }

    /// Adds a new node as the **right child** of the specified parent node.
    ///
    /// This is a convenience wrapper around [`add_node`] with `is_left` set to `true`.
    ///
    /// # Arguments
    ///
    /// * `parent_id` - The index of the parent node.
    /// * `key` - The value to store in the new node.
    ///
    /// # Returns
    ///
    /// Returns the index of the newly added node or an error if adding fails.
    pub fn add_node_right(&mut self, parent_id: usize, key: T) -> Result<usize, &str> {
        self.add_node(parent_id, key, false)
    }

    /// Returns a reference to the node specified by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The index of the node in the `nodes` vector.
    ///
    /// # Returns
    ///
    /// Returns `None` if the node with the given ID does not exist.
    pub fn get_node(&self, id: usize) -> Option<&Node<T>> {
        if id < self.nodes.len() {
            Some(&self.nodes[id])
        } else {
            None
        }
    }
}

impl<T> Tree<T>
where
    T: Add<Output = T> + Copy,
{
    /// Computes the sum of all node keys in the tree.
    ///
    /// # Returns
    /// Returns `None` if the tree is empty
    ///
    pub fn sum(&self) -> Option<T> {
        if self.get_node(0).is_none() {
            None
        } else {
            Some(self.rec_sum(0))
        }
    }

    /// Recursively computes the sum of the subtree rooted at `node_id`.
    ///
    /// # Returns
    ///
    /// The sum of all noe keys in the tree as a type
    ///
    /// # Panics
    ///
    /// Panics if `node_id` is invalid (should only be called internally with valid IDs)
    fn rec_sum(&self, node_id: usize) -> T {
        let node = self.get_node(node_id).unwrap();
        match (node.id_left, node.id_right) {
            (None, None) => node.key,
            (Some(s), None) | (None, Some(s)) => node.key + self.rec_sum(s),
            (Some(s0), Some(s1)) => node.key + self.rec_sum(s0) + self.rec_sum(s1),
        }
    }
}

impl<T> Tree<T>
where
    T: Ord,
{
    /// Checks if the entire tree satisfies the Binary Search Tree (BST) property.
    ///
    /// Returns `true` if the tree is empty or if all nodes satisfy BST ordering.
    ///
    pub fn is_bst(&self) -> bool {
        if self.nodes.is_empty() {
            return true; // empty tree is a BST
        }
        self.is_bst_rec(0, None, None)
    }

    /// Recursively checks if the subtree rooted at `node_id` is a BST,
    /// enforcing all keys > `min` and < `max` if those bounds are specified.
    ///
    /// # Panics
    ///
    /// Panics if `node_id` is invalid (should only be called internally).
    fn is_bst_rec(&self, node_id: usize, min: Option<&T>, max: Option<&T>) -> bool {
        let node = self.get_node(node_id).unwrap();
        let key = &node.key;

        // Check current node key against min and max bounds
        if let Some(min_key) = min
            && key < min_key
        {
            false
        } else if let Some(max_key) = max
            && key >= max_key
        {
            false
        }
        // Recursively check left subtree with updated max bound
        else if let Some(left_id) = node.id_left
            && !self.is_bst_rec(left_id, min, Some(key))
        {
            false
        }
        // Recursively check right subtree with updated min bound
        else if let Some(right_id) = node.id_right
            && !self.is_bst_rec(right_id, Some(key), max)
        {
            false
        } else {
            true
        }
    }
}

impl<T> Tree<T>
where
    T: Add<Output = T> + Ord + Copy + Sum,
{
    /// Computes the maximum path sum between any two leaves in the tree.
    ///
    /// Returns `None` if the tree is empty or contains no path between two leaves.
    pub fn max_path_sum(&self) -> Option<T> {
        if self.get_node(0).is_some() {
            self.max_path_sum_rec(0).1
        } else {
            None
        }
    }

    /// Recursively computes two values for the subtree rooted at `id`:
    ///
    /// 1. The maximum path sum from a leaf to the root of this subtree.
    /// 2. The maximum path sum between any two leaves in this subtree.
    ///
    /// Returns a tuple `(max_path_to_root, max_path_sum_between_leaves)`.
    ///
    /// # Explanation
    ///
    /// - If the node is a leaf, the max path to root is its key and there is no leaf-to-leaf path (None).
    /// - If the node has only one child, propagate the child's max path to root plus this node's key.
    /// - If the node has two children, compute max paths from both sides, and possibly update the max leaf-to-leaf sum.
    ///
    /// # Panics
    ///
    /// Panics if `id` is invalid (should only be called internally).
    fn max_path_sum_rec(&self, id: usize) -> (T, Option<T>) {
        let node = self.get_node(id).unwrap();
        let key = node.key;

        match (node.id_left, node.id_right) {
            (None, None) => {
                //leaf
                (key, None)
            }
            (Some(c), None) | (None, Some(c)) => {
                //only one child
                let (path, sum) = self.max_path_sum_rec(c);
                (path + key, sum)
            }
            (Some(c1), Some(c2)) => {
                //2 children
                let (path_l, sum_l) = self.max_path_sum_rec(c1);
                let (path_r, sum_r) = self.max_path_sum_rec(c2);

                let sum = path_l + path_r + key;
                let path = key + path_r.max(path_l);

                let max_sum = match (sum_l, sum_r) {
                    (None, None) => sum,
                    (Some(s), None) | (None, Some(s)) => sum.max(s),
                    (Some(s1), Some(s2)) => sum.max(s1).max(s2),
                };

                (path, Some(max_sum))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests basic tree creation and adding nodes.
    ///
    /// Tree structure after additions:
    /// ```
    ///       10
    ///      /  \
    ///     5    15
    ///    / \     \
    ///   3   7     20
    /// ```
    #[test]
    fn test_tree_construction_and_add_node() {
        let mut tree = Tree::with_root(10);
        assert_eq!(tree.nodes.len(), 1);

        let left_id = tree.add_node_left(0, 5).expect("Failed to add left child");
        let right_id = tree
            .add_node_right(0, 15)
            .expect("Failed to add right child");

        assert_eq!(left_id, 1);
        assert_eq!(right_id, 2);
        assert_eq!(tree.nodes.len(), 3);

        // Adding duplicate children to root should fail
        assert!(tree.add_node_left(0, 7).is_err());
        assert!(tree.add_node_right(0, 20).is_err());

        // Adding child to non-existent parent should fail
        assert!(tree.add_node_left(42, 7).is_err());

        tree.add_node_left(left_id, 3)
            .expect("Failed to add left grandchild");
        tree.add_node_right(left_id, 7)
            .expect("Failed to add right grandchild");
        tree.add_node_right(right_id, 20)
            .expect("Failed to add right grandchild");

        let root = tree.get_node(0).unwrap();
        assert_eq!(root.key, 10);
        assert_eq!(root.id_left, Some(left_id));
        assert_eq!(root.id_right, Some(right_id));
    }

    /// Tests summing all node keys in various tree shapes.
    ///
    /// Examples of tested trees:
    /// Single node:
    /// ```
    /// 1
    /// ```
    ///
    /// Three-node tree:
    /// ```
    ///   1
    ///  / \
    /// 2   3
    /// ```
    ///
    /// Five-node tree:
    /// ```
    ///     1
    ///    / \
    ///   2   3
    ///  / \
    /// 4   5
    /// ```
    #[test]
    fn test_sum_method() {
        let empty_tree: Tree<i32> = Tree { nodes: vec![] };
        assert_eq!(empty_tree.sum(), None);

        let mut tree = Tree::with_root(1);
        assert_eq!(tree.sum(), Some(1));

        let left_id = tree.add_node_left(0, 2).unwrap();
        let _ = tree.add_node_right(0, 3).unwrap();
        assert_eq!(tree.sum(), Some(6));

        tree.add_node_left(left_id, 4).unwrap();
        tree.add_node_right(left_id, 5).unwrap();
        assert_eq!(tree.sum(), Some(15));

        let mut neg_tree = Tree::with_root(-5);
        neg_tree.add_node_left(0, -10).unwrap();
        neg_tree.add_node_right(0, 15).unwrap();
        assert_eq!(neg_tree.sum(), Some(0));

        let mut float_tree = Tree::with_root(1.5);
        float_tree.add_node_left(0, 2.5).unwrap();
        float_tree.add_node_right(0, 3.0).unwrap();
        assert_eq!(float_tree.sum(), Some(7.0));
    }

    /// Tests the Binary Search Tree (BST) property checker on valid and invalid BSTs.
    ///
    /// Valid BST example:
    /// ```
    ///       10
    ///      /  \
    ///     5    15
    ///    / \     \
    ///   3   7     20
    /// ```
    ///
    /// Invalid BST examples:
    /// Left child > parent:
    /// ```
    ///   10
    ///  /
    /// 12
    /// ```
    ///
    /// Right child < parent:
    /// ```
    ///   10
    ///     \
    ///      8
    /// ```
    ///
    /// Deep violation:
    /// ```
    ///       10
    ///      /  \
    ///     5    15
    ///    / \     \
    ///   3  12     20
    /// ```
    #[test]
    fn test_is_bst_method() {
        let empty_tree: Tree<i32> = Tree { nodes: vec![] };
        assert!(empty_tree.is_bst());

        let single_tree = Tree::with_root(10);
        assert!(single_tree.is_bst());

        let mut tree = Tree::with_root(10);
        let left = tree.add_node_left(0, 5).unwrap();
        let right = tree.add_node_right(0, 15).unwrap();
        tree.add_node_left(left, 3).unwrap();
        tree.add_node_right(left, 7).unwrap();
        tree.add_node_right(right, 20).unwrap();
        assert!(tree.is_bst());

        let mut bad_tree = Tree::with_root(10);
        bad_tree.add_node_left(0, 12).unwrap();
        bad_tree.add_node_right(0, 15).unwrap();
        assert!(!bad_tree.is_bst());

        let mut bad_tree2 = Tree::with_root(10);
        bad_tree2.add_node_left(0, 5).unwrap();
        bad_tree2.add_node_right(0, 8).unwrap();
        assert!(!bad_tree2.is_bst());

        let mut deep_bad_tree = Tree::with_root(10);
        let left = deep_bad_tree.add_node_left(0, 5).unwrap();
        let right = deep_bad_tree.add_node_right(0, 15).unwrap();
        deep_bad_tree.add_node_left(left, 3).unwrap();
        deep_bad_tree.add_node_right(left, 12).unwrap();
        deep_bad_tree.add_node_right(right, 20).unwrap();
        assert!(!deep_bad_tree.is_bst());

        let mut str_tree = Tree::with_root("middle".to_string());
        str_tree.add_node_left(0, "apple".to_string()).unwrap();
        str_tree.add_node_right(0, "zebra".to_string()).unwrap();
        assert!(str_tree.is_bst());
    }

    /// Tests the maximum path sum between two leaves.
    ///
    /// Example tree:
    /// ```
    ///         10
    ///        /  \
    ///       2    10
    ///      / \     \
    ///    20   1    -25
    ///               / \
    ///              3   4
    /// ```
    #[test]
    fn test_max_path_sum_method() {
        let empty_tree: Tree<i32> = Tree { nodes: vec![] };
        assert_eq!(empty_tree.max_path_sum(), None);

        let single_tree = Tree::with_root(100);
        assert_eq!(single_tree.max_path_sum(), None);

        let mut simple_tree = Tree::with_root(10);
        simple_tree.add_node_left(0, 5).unwrap();
        simple_tree.add_node_right(0, 15).unwrap();
        assert_eq!(simple_tree.max_path_sum(), Some(30));

        let mut tree = Tree::with_root(10);
        let left = tree.add_node_left(0, 2).unwrap();
        let right = tree.add_node_right(0, 10).unwrap();
        tree.add_node_left(left, 20).unwrap();
        tree.add_node_right(left, 1).unwrap();
        let right_right = tree.add_node_right(right, -25).unwrap();
        tree.add_node_left(right_right, 3).unwrap();
        tree.add_node_right(right_right, 4).unwrap();

        assert_eq!(tree.max_path_sum(), Some(23));

        let mut neg_tree = Tree::with_root(-1);
        neg_tree.add_node_left(0, -2).unwrap();
        neg_tree.add_node_right(0, -3).unwrap();
        assert_eq!(neg_tree.max_path_sum(), Some(-6));

        let mut mixed_tree = Tree::with_root(5);
        let left = mixed_tree.add_node_left(0, -10).unwrap();
        let right = mixed_tree.add_node_right(0, 15).unwrap();
        mixed_tree.add_node_left(left, 20).unwrap();
        mixed_tree.add_node_right(left, 25).unwrap();
        mixed_tree.add_node_right(right, 30).unwrap();

        assert_eq!(mixed_tree.max_path_sum(), Some(65));
    }

    /// Tests the `get_node` method for valid and invalid node IDs.
    #[test]
    fn test_get_node() {
        let mut tree = Tree::with_root(5);
        assert!(tree.get_node(0).is_some());
        assert_eq!(tree.get_node(0).unwrap().key, 5);
        assert!(tree.get_node(1).is_none());

        tree.add_node_left(0, 3).unwrap();
        assert!(tree.get_node(1).is_some());
        assert_eq!(tree.get_node(1).unwrap().key, 3);

        tree.add_node_right(0, 7).unwrap();
        assert!(tree.get_node(2).is_some());
        assert_eq!(tree.get_node(2).unwrap().key, 7);

        assert!(tree.get_node(42).is_none());
        assert!(tree.get_node(100).is_none());
    }

    /// Tests different data types (characters, u8) to ensure generic correctness.
    #[test]
    fn test_different_types() {
        let mut char_tree = Tree::with_root('m');
        char_tree.add_node_left(0, 'a').unwrap();
        char_tree.add_node_right(0, 'z').unwrap();
        assert!(char_tree.is_bst());

        let mut u8_tree = Tree::with_root(100u8);
        u8_tree.add_node_left(0, 50u8).unwrap();
        u8_tree.add_node_right(0, 80u8).unwrap();
        assert!(!u8_tree.is_bst());
        assert_eq!(u8_tree.sum(), Some(230u8));
    }

    /// Tests error conditions such as adding to invalid parents and duplicate children.
    #[test]
    fn test_error_conditions() {
        let mut tree = Tree::with_root(10);

        assert!(tree.add_node_left(5, 20).is_err());
        assert!(tree.add_node_right(100, 30).is_err());

        tree.add_node_left(0, 5).unwrap();
        tree.add_node_right(0, 15).unwrap();

        assert!(tree.add_node_left(0, 3).is_err());
        assert!(tree.add_node_right(0, 20).is_err());

        let err = tree.add_node_left(0, 3).unwrap_err();
        assert_eq!(err, "Parent node has the left child already set");

        let err2 = tree.add_node_right(0, 20).unwrap_err();
        assert_eq!(err2, "Parent node has the left child already set");

        let err3 = tree.add_node_left(42, 1).unwrap_err();
        assert_eq!(err3, "Parent node id does not exist");
    }
}
