
use crate::data_structs::tree::TreeNode;

///
/// # Max Path Sum
///
/// Given a binary tree in which each node element contains a number. Find the maximum
/// possible path sum from one special node to another special node.
/// Note: Here special node is a node that is connected to exactly one different node.
///
/// ## Returns
/// An `Option<i32>` with the max sum, which is `None` if the tree has only one node
///
/// ## Details
/// This solution is implemented for a binary tree implemented as `TreeNode`, found in
/// `crate_base_dir/data_structs`
///
/// ## Strategy
///
/// By definition of the problem every leaf is a special node, or the root of the tree
/// can be a special node if it only has one child
///
/// The idea is to recursively traverse the tree, and for each subtree return a tuple of 2 elements:
/// - `max_path`: max path from a leaf to the root of the subtree
/// - `max_path_sum`: the max_path_sum of the subtree
///
/// Follows that if a node is a leaf, then the max_path is its value while the max_path_sum doesn't exist
///
/// When processing a subtree, we call the traversal on both childrens and we return the score of the
/// subtree as follows:
/// - `max_path`: `max(max_path_left,max_path_right) + root`
/// - `max_path`: can be updated if a subtree has exactely 2 children (if not it inherits the sum of the only child),
///   it gets updated if only if `max_path_left + max_path_right + root` is greather than both max_paths sum
///
/// We handle the possiblity of having a special node in the root, calling explicitly the traversal on the subtrees
/// and using `i32::MIN` as "discard" values for max operations
///
/// ## Complexity
/// We need to traverse the whole tree to compute the path-sum of 2 arbitrary special
/// nodes, this requires O(n) linear time.
///
/// No additional space is required
///
pub fn max_path_sum(t: &TreeNode<i32>) -> Option<i32> {
    match (t.get_left(), t.get_right()) {
        (None, None) => None,
        (Some(t0), None) | (None, Some(t0)) => {
            //consider paths from root to leaf
            let (path, sum) = max_path_rec(t0);
            Some(sum.max(path + t.val))
        }
        (Some(t0), Some(t1)) => {
            let (path_l, sum_l) = max_path_rec(t0);
            let (path_r, sum_r) = max_path_rec(t1);
            Some(sum_l.max(sum_r).max(t.val + path_l + path_r))
        }
    }
}

/*
 * Internal helper function
 */
fn max_path_rec(t: &TreeNode<i32>) -> (i32, i32) {
    match (t.get_left(), t.get_right()) {
        (None, None) => (t.val, i32::MIN),
        (Some(t0), None) | (None, Some(t0)) => {
            let (path, sum) = max_path_rec(t0);
            (t.val + path, sum)
        }
        (Some(t0), Some(t1)) => {
            let (path_l, sum_l) = max_path_rec(t0);
            let (path_r, sum_r) = max_path_rec(t1);
            let sum = path_l + path_r + t.val;
            (path_l.max(path_r) + t.val, sum.max(sum_l).max(sum_r))
        }
    }
}

#[cfg(test)]
mod test_max_path_sum {
    use super::*;
    use crate::{tree, tree_leaf, tree_left, tree_right};

    #[test]
    fn test_one_node() {
        let t = tree_leaf!(10);
        assert_eq!(max_path_sum(&t), None);
    }

    #[test]
    fn test_only_one() {
        let t = tree!(1, tree_leaf!(2), tree_leaf!(3));
        let expected = Some(6);
        assert_eq!(max_path_sum(&t), expected);
    }

    #[test]
    fn test_generic_1() {
        let t = tree!(3, tree!(4, tree_leaf!(-10), tree_leaf!(4)), tree_leaf!(5));
        let expected = Some(16);
        assert_eq!(max_path_sum(&t), expected);
    }

    #[test]
    fn test_generic_2() {
        let t = tree!(
            -15,
            tree!(5, tree!(-8, tree_leaf!(2), tree_leaf!(-3)), tree_leaf!(1)),
            tree!(
                6,
                tree_leaf!(3),
                tree_right!(9, tree!(0, tree_leaf!(4), tree_left!(-1, tree_leaf!(10))))
            )
        );
        let expected = Some(27);
        assert_eq!(max_path_sum(&t),expected);
    }
    #[test]
    fn test_generic_3() {
        let t = tree!(3, tree!(4, tree_leaf!(-10), tree_leaf!(4)), tree_leaf!(1));
        let expected = Some(12);
        assert_eq!(max_path_sum(&t),expected);
    }
    #[test]
    fn test_leaf_to_root() {
        let t = tree_right!(1, tree_leaf!(2));
        let expected = Some(3);
        assert_eq!(max_path_sum(&t),expected);
    }
}
