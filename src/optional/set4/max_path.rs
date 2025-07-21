use crate::data_structs::tree::TreeNode;

///
/// **MAX PATH SUM**
///
/// Given a binary tree in which each node element contains a number. Find the maximum
/// possible path sum from one special node to another special node.
/// Note: Here special node is a node that is connected to exactly one different node.
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
/// Returns an `Option<i32>` that is `None` if the tree has only one node
///
/// *Time Complexity*: tree traversal O(n)
///
/// *Space Complexity*: O(1)
///
pub fn max_path(t: &TreeNode<i32>) -> Option<i32> {
    //Tuple having (max_path in subtree , sum_special_nodes in subtree)
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
