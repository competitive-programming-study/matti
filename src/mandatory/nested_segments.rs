use crate::data_structs::fenwick_tree::FenwickTree;

///# Nested Segments
///
/// Given an array of n segments as a tuple (i64,i64) report for each segments
/// the number of other segments it contains. (Endpoints don't match for any segments)
///
/// ## Return
/// An array of usize with the count for each segment
///
/// ## Fenwick Tree Approach
/// for the i'th segment we have to count the number of segments j such that l_i < l_j
/// and r_i > r_j.
///
/// We initialize the fenwick tree with a number of cells equal to max.r - min.r then we
/// sort the segments by l. For each r in the segment we add 1 to the tree to record the
/// end of the segment.
///
/// When processing each segment, we query sum (r-1) to get the number of segments, then
/// we add -1 to r to not account for the current segment when processing the next segment
/// 
/// ## Complexity
/// We sort the semgment, then for each segment we perform fenwick tree operations so the
/// runtime is O(nlog(n))
/// 
/// We require O(n) space to store the fenwick tree
///
pub fn nested_segments(segs: &[(i32, i32)]) -> Vec<usize> {
    let mut res = vec![0; segs.len()];

    if segs.is_empty() {
        return res;
    }

    let (mut min, mut max) = (i32::MAX, i32::MIN);
    //compute min and max right-points
    for &(_, r) in segs {
        min = min.min(r);
        max = max.max(r);
    }

    //build the fenwick tree
    let ln = (max - min + 2) as usize;
    let mut ft: FenwickTree<i32> = FenwickTree::with_len(ln, 0);

    for &(_, r) in segs {
        let index = (r + 1 - min) as usize;
        let _ = ft.add(index, 1); //account for each segment end
    }

    let mut segs_sorted: Vec<(i32, i32, usize)> = segs
        .to_vec()
        .iter()
        .enumerate()
        .map(|(i, &(l, r))| (l, r, i))
        .collect(); //to keep indexing intact before sort
    segs_sorted.sort(); //lexicographic ordering for tuples (increasing for left element)

    for (_, r, i) in segs_sorted {
        let index = (r - min) as usize;
        res[i] = ft.sum(index).unwrap() as usize; //all segments that end before this 
        let _ = ft.add(index + 1, -1); //sub 1 to remove the segment that just ended
    }

    res
}
