///
/// ## WOODCUTTERS
///
/// We're given an arrya of trees, each tree is represented as a tuple (pos: i32, height: usize).
///
/// Each tree can be cutted left or right, if the cut doesn't overlap with another tree
/// (cutted or not). If no trees before, the current can always be felled left, same thing
/// for last that can always be felled right
///
/// ## Returns
/// The max number of trees we can cut
///
/// ## Strategy
/// It's a maximization problem, so we start sorting the trees by position ascending.
/// When processing each tree, we cut it left if it the position - height is higher than
/// the last recorded position. We cut right if it doesn't intersect the next tree.
///
/// We update the last recorded position accordingly. Since previous tree cuts can't overlap with
/// the current tree, we update the last recorded position with:
/// - position of the tree if we're not cutting
/// - position of the tree if we're cutting left
/// - position of the tree + height if cutting right
///
/// ## Complexity
/// We require O(nlog(n)) for sorting + one iteration
///
pub fn woodcutters(trees: &[(i64, usize)]) -> usize {
    let (mut previous, mut count) = (i64::MIN, 0);
    let mut tree_vec = trees.to_vec();
    tree_vec.sort_by_key(|t| t.0);

    for (i, &(pos, height)) in tree_vec.iter().enumerate() {
        let cut_left = pos - height as i64;
        let cut_right = pos + height as i64;
        let next = if i == tree_vec.len() - 1 {
            i64::MAX
        } else {
            tree_vec[i + 1].0
        };

        if previous < cut_left {
            //cutting left
            count += 1;
            previous = pos;
        } else if next > cut_right {
            //cutting right
            count += 1;
            previous = cut_right;
        } else {
            //not cutting
            previous = pos;
        }
    }
    count
}
