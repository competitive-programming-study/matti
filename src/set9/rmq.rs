use crate::data_structs::segment_tree::SegmentTree;

///
/// ## RMQ
///
/// We're given a circular array and we want to support 2 kinds of operations:
/// - `inc(l,r,v)` that increases each segment in the range `[l,r]` by `v`
/// - `rmq(l,r)` computes and returns the minimal value on the segment `[l,r]`
/// ### Example
/// We assume the segments to be circular so given a segment of length `5`
///
/// Given the range `[3,1]` we include the index sequence `[3,4,0,1]`
///
/// ### Parameters
/// The function takes an integer slice of arbitrary length, as well as an array of queries.
/// Queries are represented as `(l,r,Option<i64>)`, if the option is `Some(v)` then it performs
/// an `inc(v)` operation on the corresponding range, esle if `None` returns the result of the `rmq`
/// query
///
/// ### Returns
/// A vector of integers, one for each `rmq` query while performing the `inc` operations
///
/// ### Strategy
/// Since the array can be large (at most 200000 entries) we can use a segment tree
/// that supports lazy propagation. The propagation doesn't impact asymptotic complexity
/// at the worst case but can be a neat optimization that consents to compute on demand
/// updates on the tree when we're operating on partial overlapping intervals.
///
/// Since the segments are handled circularly we have to correctly handle 2 kinds
/// of intervals:
/// - [l,r] where l <= r
/// - [l,r] where l > r
///
/// The first interval can be handled internally by the segment tree implementation.
/// For the second we can apply an interval decomposition where we consider:
/// - [l,r] (where l > r) as [l,n-1] u [0,r] (inclusive)
///
/// Splitting these we can feed the query operations to the segment tree and compose the
/// result in the case of `rmq`. For add we just feed the decomposed interval since we don't
/// have to return any result.
///
/// #### Segment Tree Implementation
/// The problem requires us handling ranges of integers, so we can come up with a SegmentTree<i64>.
/// i32 would have served us good, since the range of representability includes the max and min elements (10^6)
/// but we choose i64 for compatibility and wider range.
///
/// To implement the lazy propagation we need double the memory required for the tree which normally would be 2n (so 4n)
/// because we need an additional tree where we store updates that will be performed later on. The additional tree (`lazy` from now on)
/// initially stores 0 values.
/// ##### Building the Tree
/// We build the tree using a recursive method, an iterative approach is possible but for small trees it's preferrable to do it like so
///
/// #### Handling the queries
/// For both queries, we provide an initial range (base of the recursion) as well as the range on where we want to perform the query.
///
/// Since the operations we need to provide are 2 we can safely say that by structure:
/// - the segment tree will store the array elements as leaves, and partial min query results on the internal nodes
/// - the lazy tree will store values that still need to be added onto subtrees (we will add them when we need consistency for the first query)
///
/// Both operations are recursive and we handle them considering the range (for the recursion) and the range of the query.
/// We consider 3 scenarios:
/// - no overlap: the range of the query doesn't overlap with the range of the recursion (for `inc` we return, and for `min` we return `i64::MAX`)
/// - total overlap: the range of the query is fully included by the range of the recursion (we return the current node: can be leaf or intermediate)
/// - partial overlap: we have to recursively consider both subtrees: (we need constency) so all lazy updates have to be committed before this. Then we
/// call the recursion on the subtrees (in the case of `min` query we then compose the result as `min(rec_left,rec_right)`)
/// #### Setting lazy propagation
/// For the add query we can store intermediate results (in the case of total overlap) in the lazy tree.
///
/// If a subsequent `min` query needs a result of a range that wasn't committed yet, then calls `push` that checks the lazy tree
/// and "push" the updated value to the lazy node children as well as the tree children. With this we guarantee that (at least for the
/// first recursive call) the intermediate query is consistent. Since every recursive call can call `push` this is an invariant of the
/// data structure that guarantees consistency on every level of recursion
///
pub fn rmq(ring: &[i64], query: &[(usize, usize, Option<i64>)]) -> Vec<i64> {
    let mut tree = SegmentTree::build(ring);
    let mut min_queries = 0;
    for &(_, _, q) in query {
        if let None = q {
            min_queries += 1;
        }
    }

    if min_queries == 0 {
        return vec![];
    }
    let mut res = Vec::with_capacity(min_queries);

    for &(l, r, q) in query {
        if let Some(v) = q {
            tree.add_range(l, r, v);
        } else {
            res.push(tree.min_range(l, r));
        }
    }
    res
}
