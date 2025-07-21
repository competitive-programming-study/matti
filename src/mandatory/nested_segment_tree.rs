use crate::data_structs::segment_tree::SegmentTreeSum;

///# Nested Segments
///
/// Given an array of n segments as a tuple (i64,i64) report for each segments
/// the number of other segments it contains. (Endpoints don't match for any segments)
///
/// ## Return
/// An array of usize with the count for each segment
///
/// ## Segment Tree Approach
/// for the i'th segment we have to count the number of segments j such that l_i < l_j
/// and r_i > r_j.
///
/// We initialize the segment tree with a number of cells equal to max.r - min.r then we
/// sort the segments by l. For each r in the segment we add 1 to the tree to record the
/// end of the segment.
///
/// When processing each segment, we query range_sum (r-1) to get the number of segments, then
/// we add -1 to `r` to not account for the current segment when processing the next segment
///
/// ## Complexity
/// We sort the semgment, then for each segment we perform segment tree operations so the
/// runtime is O(nlog(n))
///
/// We require O(n) space to store the fenwick tree
///
pub fn nested_segments(segs: &[(i32, i32)]) -> Vec<usize> {
    let mut res = vec![0; segs.len()];

    if segs.is_empty() {
        return res;
    }

    // 1. Normalize coordinate space by getting min and max r
    let (mut min_r, mut max_r) = (i32::MAX, i32::MIN);
    for &(_, r) in segs {
        min_r = min_r.min(r);
        max_r = max_r.max(r);
    }

    // 2. Size of segment tree based on compressed r-coordinates
    let size = (max_r - min_r + 1) as usize;
    let mut st: SegmentTreeSum<i32> = SegmentTreeSum::build(&vec![0; size]);

    // 3. Add 1 at each segment's right endpoint
    for &(_, r) in segs {
        let index = (r - min_r) as usize;
        st.range_add(index, index, 1); // mark where segments end
    }

    // 4. Sort by left endpoint (increasing), ties broken by r (doesn't matter since no overlap)
    let mut segs_sorted: Vec<(i32, i32, usize)> = segs
        .iter()
        .enumerate()
        .map(|(i, &(l, r))| (l, r, i))
        .collect();
    segs_sorted.sort_by_key(|&(l, _, _)| l);

    // 5. For each segment, count segments that end before it (r_j < r_i)
    for (_, r, i) in segs_sorted {
        let index = (r - min_r) as usize;
        if index > 0 {
            res[i] = st.range_sum(0, index - 1) as usize;
        } else {
            res[i] = 0;
        }

        // Remove this segment’s right endpoint so it doesn’t affect future counts
        st.range_add(index, index, -1);
    }

    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let segs: Vec<(i32, i32)> = vec![];
        let expected: Vec<usize> = vec![];
        assert_eq!(nested_segments(&segs), expected);
    }

    #[test]
    fn test_single_segment() {
        let segs = vec![(1, 5)];
        let expected = vec![0];
        assert_eq!(nested_segments(&segs), expected);
    }

    #[test]
    fn test_two_non_overlapping_segments() {
        let segs = vec![(1, 3), (4, 6)];
        let expected = vec![0, 0];
        assert_eq!(nested_segments(&segs), expected);
    }

    #[test]
    fn test_two_nested_segments() {
        let segs = vec![(1, 10), (3, 7)];
        let expected = vec![1, 0]; // First contains second
        assert_eq!(nested_segments(&segs), expected);
    }

    #[test]
    fn test_multiple_nested_segments() {
        let segs = vec![(1, 10), (2, 9), (3, 8), (4, 7)];
        let expected = vec![3, 2, 1, 0];
        assert_eq!(nested_segments(&segs), expected);
    }

    #[test]
    fn test_reverse_ordered_input() {
        let segs = vec![(4, 7), (3, 8), (2, 9), (1, 10)];
        let expected = vec![0, 1, 2, 3]; // Order doesn't affect correctness
        assert_eq!(nested_segments(&segs), expected);
    }

    #[test]
    fn test_disjoint_and_nested_mix() {
        let segs = vec![(1, 5), (6, 10), (3, 4), (2, 9)];
        // Index 0 contains 2, index 3 contains 2
        let expected = vec![1, 0, 0, 1];
        assert_eq!(nested_segments(&segs), expected);
    }

    #[test]
    fn test_overlapping_but_not_nested() {
        let segs = vec![(1, 5), (3, 7), (2, 6)];
        let expected = vec![0, 0, 0]; // No segment fully contains another
        assert_eq!(nested_segments(&segs), expected);
    }

}
