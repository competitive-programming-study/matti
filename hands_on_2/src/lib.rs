mod data_structs {
    use std::collections::HashSet;
    use std::fmt::Debug;
    use std::hash::Hash;

    pub struct SegmentTreeMinMax<T>
    where
        T: PartialOrd + Ord + Copy,
    {
        pub(crate) n: usize,
        pub(crate) tree: Vec<Option<T>>,
        pub(crate) lazy: Vec<Option<T>>,
    }

    impl<T> SegmentTreeMinMax<T>
    where
        T: PartialOrd + Ord + Copy,
    {
        #[allow(unused)]
        ///
        ///  Returns the number of leaves in the Segment Tree.
        ///
        ///  Useful if we want a range on the whole tree
        pub fn len(&self) -> usize {
            self.n
        }

        #[allow(unused)]
        pub fn is_empty(&self) -> bool {
            self.n == 0
        }

        ///
        /// Merges 2 children nodes into a single one based on the update procedure
        ///
        /// Used for parent node recomputation in building the tree or after an update
        ///
        fn merge_segments(left: Option<T>, right: Option<T>) -> Option<T> {
            match (left, right) {
                (None, None) => None,
                (Some(ml), Some(mr)) => Some(ml.max(mr)),
                (Some(m), _) | (_, Some(m)) => Some(m),
            }
        }

        ///
        /// Recursive procedure that builds a Segment tree from a generic slice
        ///
        /// The procedure uses feedback to update parent nodes after the recursion has
        /// been unraveled
        ///
        fn build_rec(
            a: &[T],
            seg: &mut [Option<T>],
            idx: usize,
            left: usize,
            right: usize,
        ) -> Option<T> {
            if idx >= seg.len() {
                return None;
            }

            if left == right {
                seg[idx] = Some(a[left]);
                return seg[idx];
            }

            let mid = (left + right) / 2;
            let c0 = Self::build_rec(a, seg, idx * 2 + 1, left, mid);
            let c1 = Self::build_rec(a, seg, idx * 2 + 2, mid + 1, right);

            seg[idx] = Self::merge_segments(c0, c1);
            seg[idx]
        }

        ///
        /// Builds a segment tree that supports lazy propagation from a generic slice
        ///
        /// ## Returns
        /// The segment tree
        ///
        /// ## Notes
        /// The segment tree is built on generic <T> values and internally uses Option<_> as nodes to avoid
        /// default values.
        ///
        /// Lazy propagation support doubles the amount of memory required. Without would be `4*n` -> with is `8*n`
        pub fn from_slice(a: &[T]) -> Self {
            let ln = (2 * a.len().next_power_of_two()) - 1;
            let mut tree = vec![None; ln];
            let lazy: Vec<Option<T>> = vec![None; ln];
            let _ = Self::build_rec(a, &mut tree, 0, 0, a.len() - 1);
            Self {
                n: a.len(),
                tree,
                lazy,
            }
        }

        /// Push the pending update stored in `lazy[idx]` down to the two children.
        ///
        /// Invariant:
        ///   * `tree[i] = Some((min_i, max_i))`  stores the segment's (min, max)
        ///   * `lazy[i] = Some(t)` means "every element in this segment is still
        ///     waiting for x <- min(x, t)".
        ///
        /// After `push(idx)`
        ///   * `lazy[idx]` is cleared,
        ///   * each child `j` receives min(lazy[j],t)
        fn push_updates(&mut self, idx: usize) {
            // Nothing to propagate?
            let Some(t) = self.lazy[idx] else {
                return;
            };
            // Children indices
            let (left, right) = (idx * 2 + 1, idx * 2 + 2);
            for child in [left, right] {
                // update the lazy tag
                self.lazy[child] = match self.lazy[child] {
                    None => Some(t),
                    Some(old_t) => Some(old_t.min(t)),
                };

                // update the child tree node
                if let Some(ref mut seg) = self.tree[child]
                    && *seg > t
                {
                    *seg = t;
                }
            }

            // Clear the parent's tag: it has been fully pushed.
            self.lazy[idx] = None;
        }

        /// Internal recursive function that gets called when needing to recursively
        /// update nodes of the tree.
        ///
        /// Provided a range query (qleft,qright) and a value: T, it recursively updates
        /// the nodes in the range if the value is lower than the current. For internal
        /// nodes may update (min,max), while for leaves it updates the whole tuple.
        ///
        /// Leaves are stored as` (min,max) where min = max = v`;
        ///
        /// It uses lazy propagation to optimize large range queries
        /// Updates the range `[qleft, qright]` with `val` and
        /// returns the segment’s (min,max) after the update.
        ///
        /// ## Notes
        ///
        /// Doesn't perform checks on the range sanity, should be called internally
        fn range_update_query(
            &mut self,
            idx: usize,
            qleft: usize,
            qright: usize,
            left: usize,
            right: usize,
            val: T,
        ) -> Option<T> {
            if qleft > right || qright < left {
                // No overlap: no change
                // Just return current node value
            } else if qleft <= left && qright >= right {
                // Total overlap:
                // Conditionally update the node and the lazy one
                // or discard the update if redundant
                if let Some(ref mut max) = self.tree[idx]
                    && *max > val
                {
                    *max = val;
                    if let Some(ref mut lz) = self.lazy[idx]
                        && *lz > val
                    {
                        *lz = val;
                    } else {
                        self.lazy[idx] = Some(val);
                    }
                }
            } else {
                // Partial overlap: push lazy, recursive call on children then update the node
                self.push_updates(idx);
                let mid = (left + right) / 2;
                let left = self.range_update_query(idx * 2 + 1, qleft, qright, left, mid, val);
                let right =
                    self.range_update_query(idx * 2 + 2, qleft, qright, mid + 1, right, val);
                self.tree[idx] = Self::merge_segments(left, right);
            }

            self.tree[idx]
        }

        /// Internal recursive function that gets called when needing to recursively
        /// search the max in a range
        ///
        /// Provided a range query (qleft,qright) and a value: T, it returns the max
        /// value in the range provided
        ///
        /// Leaves are stored as` (min,max) where min = max = v`;
        ///
        /// It may trigger lazy propagation to optimize large range queries
        /// Updates the range `[qleft, qright]` with `val` and
        /// returns the segment’s (min,max) after the update.
        ///
        /// ## Notes
        ///
        /// Doesn't perform checks on the range sanity, should be called internally
        fn range_max_query(
            &mut self,
            idx: usize,
            qleft: usize,
            qright: usize,
            left: usize,
            right: usize,
        ) -> Option<T> {
            //No overlap
            if qleft > right || qright < left {
                None
            }
            //Total overlap
            else if qleft <= left && qright >= right {
                //we have to update the tree and the lazy
                self.tree[idx]
            }
            //Partial overlap
            else {
                self.push_updates(idx); //push lazy tree updates to lower level

                let mid = (left + right) / 2;
                let child = idx * 2 + 1;
                let ch0 = self.range_max_query(child, qleft, qright, left, mid);
                let ch1 = self.range_max_query(child + 1, qleft, qright, mid + 1, right);

                match (ch0, ch1) {
                    (None, None) => None,
                    (Some(v), None) | (None, Some(v)) => Some(v),
                    (Some(v0), Some(v1)) => Some(v0.max(v1)),
                }
            }
        }

        ///
        /// Checks if the user-provided range is supported.
        ///
        /// ## Note
        ///
        /// Not currently supported circular ranges
        pub fn range_sanity(&self, left: usize, right: usize) -> bool {
            left <= right && right < self.n
        }

        #[allow(unused)]
        ///
        /// Returns the value of a leaf
        ///
        /// ## Note
        ///
        /// Internally calls the range_max_query providing a range of amplitude one
        ///
        /// May trigger lazy propagation of pending updates
        ///
        /// Used only for test purposes but user-safe
        pub fn get(&mut self, idx: usize) -> Option<T> {
            if idx < self.n {
                self.range_max_query(0, idx, idx, 0, self.n - 1)
            } else {
                None
            }
        }

        /// User-API method to query the max value in a range
        ///
        /// ## Returns
        /// An `Option<T>` which is `None` if the range provided is not supported (see `range_sanity(...)`)
        pub fn max(&mut self, left: usize, right: usize) -> Option<T> {
            if self.range_sanity(left, right) {
                self.range_max_query(0, left, right, 0, self.n - 1)
            } else {
                None
            }
        }

        /// User-API method to update values in a range
        ///
        /// ## Specs
        /// The update procedure requires a range of operation `[left <= right <= self.len()]` and updates
        /// all values in the range to `min(val,current_value)`
        ///
        /// ## Returns
        /// A `Result<()>` is `Err(...)` if the range provided is not supported (see `range_sanity(...)`)
        pub fn update(&mut self, left: usize, right: usize, val: T) -> Result<(), &str> {
            if self.range_sanity(left, right) {
                self.range_update_query(0, left, right, 0, self.n - 1, val);
                Ok(())
            } else {
                Err("Range provided is not supported")
            }
        }
    }

    #[derive(Clone, Debug)]
    ///
    /// Generic Enum used to optimize storage of keys in the SegmentTree
    pub(crate) enum Coverage<T> {
        Set(HashSet<T>),
        Val(T),
    }

    ///
    /// ## SegmentTreeHash
    ///
    /// Particular Segment Tree implementation that allows in O(log(n)) time to check if a value exists within
    /// a range
    ///
    /// The tree is designed to be immutable after construction, so it is only optimized to allow for existence of
    /// a value based queries in a range
    ///
    /// ### Time Complexity
    /// At most `O(log(n))` recursive calls per query. Other than the number of recursive calls, the cost per call
    /// is constant (it uses `HashSet<>::contains()` which has constant time complexity, or direct value comparison)
    ///
    /// ### Space Complexity
    /// Uses `HashSet<T>` to store the keys present in the subtree. This dictates that the space complexity is
    /// O(n*log(n)).
    ///
    /// Assuming the worst case where all keys (n) in a range are distinct, the last level of the key stores `4*n`
    /// keys (to account for balancing values). For every internal level we store the merged `HashSet<>` of the
    /// subtree. This means that (in the case of n distinct trees) for every level we store `n` keys (asymptotic
    /// complexity so we don't account the HashSet overhead). The tree is balanced so we have `O(log(n))` levels,
    ///
    pub struct SegmentTreeHash<T>
    where
        T: PartialEq + Eq + Hash + Clone + Copy,
    {
        pub(crate) n: usize,
        pub(crate) tree: Vec<Option<Coverage<T>>>,
    }

    impl<T> SegmentTreeHash<T>
    where
        T: PartialEq + Eq + Hash + Clone + Copy,
    {
        ///
        /// Merges 2 `Coverage<T>` into a single one. `Coverage<T>` is an `enum` that can store either
        /// a `HashSet<T>` or a single `T` value.
        ///
        /// The merge procedure works by cases, having 2 `Coverage::Set` we will compute the merge set,
        /// having two values we will compute a Coverage::Value (if the value provided are equal) or
        /// a `Coverage::Set` of the 2 values otherwise. So on for the other cases.
        ///
        /// ### Time Complexity
        /// It has a complexity of O(a + b) in merging 2 `HashSet` of respective sizes `a,b`
        ///
        fn merge_coverage(left: &Coverage<T>, right: &Coverage<T>) -> Coverage<T> {
            match (left, right) {
                (Coverage::Set(h0), Coverage::Set(h1)) => {
                    // Pick the larger set as the one to fill
                    let (mut to_fill, filler) = if h0.len() > h1.len() {
                        (h0.clone(), h1)
                    } else {
                        (h1.clone(), h0)
                    };
                    for e in filler.iter() {
                        to_fill.insert(*e);
                    }
                    Coverage::Set(to_fill)
                }
                //if the two values are the same return the value
                (Coverage::Val(v0), Coverage::Val(v1)) => {
                    if v0 == v1 {
                        Coverage::Val(*v0)
                    } else {
                        let mut set = HashSet::new();
                        set.insert(*v0);
                        set.insert(*v1);
                        Coverage::Set(set)
                    }
                }
                (Coverage::Val(v), Coverage::Set(h)) | (Coverage::Set(h), Coverage::Val(v)) => {
                    let mut set = h.clone();
                    set.insert(*v);
                    Coverage::Set(set)
                }
            }
        }

        ///
        /// Merges 2 nodes into a single one
        ///
        /// Useful for parent node construction in building the tree.
        ///
        /// Nodes in the tree are stored as `Option<Coverage<T>>`. Internally may calls `merge_coverage(...)`
        fn merge_segments(
            seg: &[Option<Coverage<T>>],
            left: usize,
            right: usize,
        ) -> Option<Coverage<T>> {
            let left_cov = if left < seg.len() { &seg[left] } else { &None };
            let right_cov = if right < seg.len() {
                &seg[right]
            } else {
                &None
            };

            match (left_cov, right_cov) {
                (None, None) => None,
                (None, Some(v)) | (Some(v), None) => Some(v.clone()),
                (Some(v1), Some(v2)) => Some(Self::merge_coverage(v1, v2)),
            }
        }

        ///
        /// Recursively builds the tree from a generic slice
        ///
        ///
        fn build_rec(
            a: &[T],
            seg: &mut [Option<Coverage<T>>],
            idx: usize,
            left: usize,
            right: usize,
        ) {
            if idx >= seg.len() {
            } else if left == right {
                seg[idx] = Some(Coverage::Val(a[left]));
            } else {
                let mid = (left + right) / 2;
                let (c0, c1) = (idx * 2 + 1, idx * 2 + 2);
                Self::build_rec(a, seg, c0, left, mid);
                Self::build_rec(a, seg, c1, mid + 1, right);
                seg[idx] = Self::merge_segments(seg, c0, c1);
            }
        }

        ///
        /// Builds a `SegmentTreeHash<T>` from a generic slice
        ///
        /// ## Time Complexity
        ///
        /// The tree is designed to store (at the worst case) for every subtree the combined hash set of
        /// every range value.
        ///
        /// This means that, for every recursive call we have a merge step of `O(n)`. Since the tree is balanced
        /// we have at most `log(n)` divide that require a linear time merge step. This dictates that the build
        /// time complexity is `O(nlog(n))`;
        pub fn from_slice(slice: &[T]) -> Self {
            let ln = (2 * slice.len().next_power_of_two()) - 1;
            let mut tree = vec![None; ln];
            Self::build_rec(slice, &mut tree, 0, 0, slice.len() - 1);
            Self {
                n: slice.len(),
                tree,
            }
        }

        ///
        /// Recursive function that checks if a provided key exists in a range
        ///
        /// ### Notes
        /// Doesn't perform sanity checks in the range (should only be used internally)
        fn is_there_rec(
            &self,
            qleft: usize,
            qright: usize,
            idx: usize,
            left: usize,
            right: usize,
            val: T,
        ) -> bool {
            if qleft > right || qright < left {
                //No overlap
                false
            } else if qleft <= left && qright >= right {
                //Total overlap: we check if the current node contains or is equal to the value provided
                if let Some(coverage) = &self.tree[idx] {
                    match coverage {
                        Coverage::Set(h) => h.contains(&val),
                        Coverage::Val(v) => *v == val,
                    }
                } else {
                    false
                }
            } else {
                //Partial overlap we need to check in both intervals
                let mid = (left + right) / 2;
                self.is_there_rec(qleft, qright, (idx * 2) + 1, left, mid, val)
                    || self.is_there_rec(qleft, qright, (idx * 2) + 2, mid + 1, right, val)
            }
        }

        pub fn is_there(&self, left: usize, right: usize, val: T) -> Result<bool, &str> {
            if left <= right && right < self.n {
                Ok(self.is_there_rec(left, right, 0, 0, self.n - 1, val))
            } else {
                Err("Range provided is not supported")
            }
        }
    }
}

#[allow(unused, dead_code)]
pub mod min_max {
    use crate::data_structs::SegmentTreeMinMax;

    ///
    /// Enums that defines possible queries for the `SegmentMinMax` data structure
    #[derive(Debug, PartialEq)]
    pub enum Query<T> {
        Update((usize, usize, T)),
        Max((usize, usize)),
    }

    ///
    /// Public User-API method to process a range of Queries given an initial range of values
    ///
    /// ## Parameters
    /// - A slice of a generic `T` type that satisfies the trait restrictions of `SegmentTreeMinMax`
    /// - A slice of `Query<T>`
    ///
    /// ## Returns
    /// A vector of `Option<T>`. The i-th if `Some(v)` is the correct answer to the i-th query
    /// If `None` then the query coudn't be performed because the range was malformed
    ///
    /// ## Notes
    /// Since it was mandatory to support 1-based range queries, the `solve` methods normalizes them to
    /// 0-range intervals before feeding them to the `SegmentTreeMinMax`
    ///
    /// ## Space Complexity (linear)
    /// For each solve call,  a `SegmentTreeMinMax` gets build from the slice provided. Given a slice of
    /// `n` items then the space required by the SegmentTree is `8n => O(n)`
    ///
    /// ## Time Complexity
    /// We can divide the `solve` method into 2 steps:
    /// - `SegmentTreeMinMax` construction: which takes O(n) time: O(n) for default initialization + O(n) for inititalization
    /// - Query answer: provided we supply m queries, and the time complexity for a query to the SegmentTree is Omega(log(n)), the
    ///   time complexity for this part is `m*log(n)`
    ///
    /// So the total time complexity is O(n + mlog(n));
    ///
    pub fn solve<T: Ord + Copy>(a: &[T], queries: &[Query<T>]) -> Vec<Option<T>> {
        let mut t = SegmentTreeMinMax::from_slice(a);

        let len = queries
            .iter()
            .filter(|&item| matches!(item, Query::Max((_, _))))
            .count();

        let mut ans = Vec::with_capacity(len);
        for q in queries.iter() {
            match q {
                Query::Update((left, right, val)) => {
                    let _ = t.update(*left - 1, *right - 1, *val);
                }
                Query::Max((left, right)) => ans.push(t.max(*left - 1, *right - 1)),
            };
        }
        ans
    }

    ///
    /// Parses an input string to an array of usizes and an array of queries.
    ///
    pub fn parse_input(input: &str) -> (Vec<usize>, Vec<Query<usize>>) {
        let mut lines = input.lines();
        // Parse n and m
        let first_line = lines.next().unwrap();
        let mut parts = first_line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let _m: usize = parts.next().unwrap().parse().unwrap(); // m is not strictly needed
        // Parse the array A
        let array_line = lines.next().unwrap();
        let array: Vec<usize> = array_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(array.len(), n, "Failed parsing input array");
        // Parse the queries
        let queries: Vec<Query<usize>> = lines
            .map(|line| {
                let mut tokens = line.split_whitespace();
                let kind: usize = tokens.next().unwrap().parse().unwrap();
                let i: usize = tokens.next().unwrap().parse().unwrap();
                let j: usize = tokens.next().unwrap().parse().unwrap();
                match kind {
                    0 => Query::Update((i, j, tokens.next().unwrap().parse().unwrap())),
                    1 => Query::Max((i, j)),
                    _ => panic!("Query format doesn't match neither Max(i,j) nor Update(i,j,v)"),
                }
            })
            .collect();
        assert_eq!(_m, queries.len(), "Failed parsing input queries");
        (array, queries)
    }

    ///
    /// Parses an output string to `Vec<usize>` for results comparison
    pub fn parse_output(output: &str) -> Vec<Option<usize>> {
        let lines = output.lines();
        let mut out = vec![];
        for l in lines {
            out.push(Some(l.parse().unwrap()));
        }
        out
    }
}

#[allow(unused)]
pub mod is_there {
    pub type Segment = (usize, usize);
    pub type Query = (usize, usize, usize);
    use crate::data_structs::SegmentTreeHash;

    ///
    /// Builds an unsigned integer coverage intervals from a list of segments
    ///
    /// Given `n` possibly overlapping segments in the range `[0,n-1]` it returns a coverage interval
    /// from [0,n-1] where for every position it counts how many segments cover that position.
    ///
    /// To do this we use a support array where for every segment, we mark with +1  the point of start and -1
    /// the first point after the end.
    ///
    /// Then we iterate on the support array computing the prefix sum.
    ///
    /// A different approach would have been to use a `Fenwick tree`` but woudn't have impacted complexity
    /// and would have made the method more cumbersome due to casting
    ///
    /// ## Time complexity
    /// Since the support array and and the number of segments have the same size, the complexity is linear
    /// in the number of segments provided.
    fn coverage_interval(segments: &[Segment]) -> Vec<usize> {
        let mut diff = vec![0; segments.len()];
        for &(s, e) in segments {
            diff[s] += 1;
            if e + 1 < diff.len() {
                diff[e + 1] -= 1;
            }
        }
        for i in 1..diff.len() {
            diff[i] += diff[i - 1];
        }
        diff.iter().map(|&x| x as usize).collect()
    }

    ///
    /// Public User-API method that returns the answer to a batch of queries provided a slice of segments
    ///
    /// ### Notes
    /// It uses a segment tree that for each range contains a hashset with all the existing values in that
    /// particular range.
    ///
    /// ### Parameters
    /// - A slice of n intervals in the range [0,n-1]
    /// - A slice of m 0-index based queries
    ///
    /// ### Returns
    /// A vector of `Option<bool>`. The i-th item, if `Some(v)` is the answer to the i-th query.
    ///
    /// If `None` then the range provided for the i-th query was malformed.
    ///
    /// ### Time Complexity
    /// The method can be divided in 3 steps:
    /// - compute coverage interval: `O(n)`
    /// - build `SegmentTreeHash`: `O(nlog(n))` (Best Case O(n))
    /// - compute the answer for each query: `O(mlog(n))`
    ///
    /// So the total time complexity is `O(n+mlog(n))`
    pub fn solve_segment_tree(segs: &[Segment], queries: &[Query]) -> Vec<Option<bool>> {
        let t = SegmentTreeHash::from_slice(&coverage_interval(segs));
        let mut ans = Vec::with_capacity(queries.len());
        for &(left, right, k) in queries {
            let res = t.is_there(left, right, k);
            ans.push(if res.is_ok() { res.ok() } else { None });
        }
        ans
    }

    ///
    /// Standard binary search that given a slice and a range returns
    /// true if at least one item falls under the range provided
    fn range_bin_search(a: &[usize], i: usize, j: usize) -> bool {
        let (mut left, mut right) = (0, a.len());

        // Find the smallest element >= i using manual binary search
        while left < right {
            let mid = left + (right - left) / 2;
            if a[mid] < i {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left < a.len() && a[left] <= j
    }

    ///
    /// Public User-API method that returns the answer to a batch of queries provided a slice of segments
    ///
    /// ### Notes
    /// IsThere(i,j,k) ask if there exists a point p in the range i,j covered by exactely k segments
    ///
    /// the previous method achieves the same functionality, by storing in a hashmap all possible values
    /// of p in a specific range. While this is straighforward requires a lot of space and duplication of
    /// items, plus it relies on HashSets. While operations on average are executed in constant time, we
    /// can't expect (by modern implementation) to achieve an upper bound that is constant.
    ///
    /// This approach relies much more on preprocessing, producing a data structure that allows for fast
    /// lookup operations (log(n)) without relying on segment trees, nor on hashing.
    ///
    /// The idea is to compute the coverage interval. Having n segments in the range [0,n-1] we can
    /// preallocate a vector (lookup) for each different overlapping. we can then map each coverage, to a tuple,
    /// of the form (coverage_i,i). With this, for each coverage we insert (sorted) the points that have
    /// that specific coverage inside a vector, that we will store in the corresponding lookup[coverage] position
    ///
    /// With this we can directly look up for the supposed k position, and binary (reverse) search the p point
    /// providing the range
    ///
    /// ### Time Complexity
    /// - computing the coverage interval: O(n) + sorting O(nlog(n))
    /// - insert all items in the specific k vectors: O(n)
    /// - lookup per query: O(1) to access the k-th vector + O(log(n)) for binary search
    ///
    /// ### Space complexity
    /// In order to avoid hashing which would have given better amortized time complexity we use an
    /// additional lookup vector to allow for worst-case O(1) insertion and lookup.
    ///
    /// We pay O(n) for the lookup vector + a sparse O(n) for each point in the interval.
    ///
    /// The Space complexity is linear in the number of segments (or the interval) we have to cover
    pub fn solve_binary_lookup(segs: &[Segment], queries: &[Query]) -> Vec<Option<bool>> {
        //build the coverage interval then map it to get the points in the interval
        //prioritize (cov,point)
        let mut cov: Vec<(usize, usize)> = coverage_interval(segs)
            .iter()
            .enumerate()
            .map(|(i, &cov)| (cov, i))
            .collect();
        //sort the tuples lexicographically (by first item, then second)
        cov.sort_unstable();

        //build the lookup table
        //we need an additional vector for each possible overlapping
        //having n segments the highest possible overlapping is a point
        //covered by all segments
        let mut lookup: Vec<Option<Vec<usize>>> = vec![None; segs.len() + 1];

        for (c, i) in cov {
            match &mut lookup[c] {
                Some(v) => v.push(i),              //push to existing vector
                None => lookup[c] = Some(vec![i]), //create a new vector with the item
            };
        }

        let mut answers: Vec<Option<bool>> = Vec::with_capacity(queries.len());

        for &(i, j, k) in queries {
            answers.push(Some(if let Some(v) = &lookup[k] {
                range_bin_search(v, i, j)
            } else {
                false
            }));
        }
        answers
    }
    ///
    /// Parses a vector of segments and queries from an input string
    ///
    pub fn parse_input(input: &str) -> (Vec<Segment>, Vec<Query>) {
        let mut lines = input.lines();

        // Parse the first line: n and m
        let first_line = lines.next().unwrap();
        let mut header = first_line.split_whitespace();
        let n: usize = header.next().unwrap().parse().unwrap();
        let m: usize = header.next().unwrap().parse().unwrap();

        // Parse next n lines: (usize, usize) segments
        let segments: Vec<(usize, usize)> = lines
            .by_ref()
            .take(n)
            .map(|line| {
                let mut parts = line.split_whitespace();
                let a = parts.next().unwrap().parse().unwrap();
                let b = parts.next().unwrap().parse().unwrap();
                (a, b)
            })
            .collect();

        // Parse next m lines: (usize, usize, usize) queries
        let queries: Vec<(usize, usize, usize)> = lines
            .take(m)
            .map(|line| {
                let mut parts = line.split_whitespace();
                let a = parts.next().unwrap().parse().unwrap();
                let b = parts.next().unwrap().parse().unwrap();
                let c = parts.next().unwrap().parse().unwrap();
                (a, b, c)
            })
            .collect();
        (segments, queries)
    }

    ///
    /// Parses an output string to a `Vec<Option<bool>>`
    ///
    /// Each `Option<bool>` is always `Some(bool)`. The option is used so that the
    /// `parse_output` output keeps the same signature as the `solve(...)` output
    /// for result comparison.
    ///
    /// Useful for test purposes and correct output confirmation
    pub fn parse_output(input: &str) -> Vec<Option<bool>> {
        input
            .lines()
            .map(|line| match line.trim() {
                "0" => false,
                "1" => true,
                other => panic!("Invalid boolean value: {other}"),
            })
            .map(Some)
            .collect()
    }
}

#[cfg(test)]
mod test_min_max {
    use crate::{data_structs::SegmentTreeMinMax, min_max::*};
    use std::env::current_dir;
    use std::fs::read_to_string;
    use std::path::PathBuf;

    ///
    /// Internal methods that returns the full path of a test input-output file
    ///
    /// All tests are located in the crate_root/test_min_max/folder.
    ///
    /// Each `input` test file is distinguished by an unique id that matches the expected `output` test file
    fn test_path(name: &str) -> PathBuf {
        current_dir().unwrap().join("test_min_max").join(name)
    }

    ///
    /// Internal method that parses input and output files and tests them against
    ///
    /// ### Panics
    /// If the parsed expected output is not the same as the result of the test on the parsed input
    fn test_files(input_name: &str, output_name: &str) {
        let input_path = test_path(input_name);
        let output_path = test_path(output_name);
        let input = read_to_string(input_path).unwrap();
        let output = read_to_string(output_path).unwrap();

        let (arr, queries) = parse_input(&input);
        let expected = parse_output(&output);
        assert_eq!(solve(&arr, &queries), expected);
    }

    ///
    /// Basic Segment tree construction test case
    #[test]
    fn test_tree_construction() {
        let a = [5, 2, 1, 7, 3, 4];
        let mut s = SegmentTreeMinMax::from_slice(&a);
        assert_eq!(s.len(), a.len());
        for (i, &e) in a.iter().enumerate() {
            let res = s.get(i);
            assert!(res.is_some_and(|v| v == e));
        }
    }

    ///
    /// Basic segment tree update and queries test case
    ///
    #[test]
    fn test_tree_basic_ops() {
        let arr = vec![5, 3, 8, 7];
        let mut st = SegmentTreeMinMax::from_slice(&arr);
        assert_eq!(st.max(0, 3), Some(8));

        st.update(1, 2, 4).unwrap(); // clamp 3 and 8 to min(x,4)
        assert_eq!(st.get(1), Some(3)); // originally 3, clamped to 3
        assert_eq!(st.get(2), Some(4)); // originally 8, clamped to 4
        assert_eq!(st.max(0, 3), Some(7)); // 7 is now max
    }

    ///
    /// Basic parsing input test case
    ///
    #[test]
    fn test_parse_input() {
        use Query::*;

        let input = [
            "5 3",       // n m
            "5 1 4 3 2", // The array A
            "0 1 2 2",   // Update(1, 2, 2). The array A becomes 2 1 4 3 2.
            "1 2 4",     // Max(2, 4) = 4
            "1 1 2",     // Max(1, 2) = 2
        ]
        .join("\n");

        let (a, q) = parse_input(&input);
        let expected_a: Vec<usize> = vec![5, 1, 4, 3, 2];
        let expected_q: Vec<Query<usize>> = vec![Update((1, 2, 2)), Max((2, 4)), Max((1, 2))];
        assert_eq!(a, expected_a);
        assert_eq!(q, expected_q);
    }

    ///
    /// Basic parse input test case
    ///
    #[test]
    fn test_parse_output() {
        let a = ["4", "2"].join("\n");
        let expected_a: Vec<Option<usize>> = vec![Some(4), Some(2)];
        assert_eq!(parse_output(&a), expected_a);
    }

    ///
    /// Full fledged test suite for min_max
    ///
    #[test]
    fn test_input_output() {
        for i in 0..=10 {
            let input = format!("input{i}.txt");
            let output = format!("output{i}.txt");
            test_files(&input, &output);
        }
    }
}

#[cfg(test)]
mod test_is_there {
    use crate::{data_structs::SegmentTreeHash, is_there::*};
    use std::env::current_dir;
    use std::fs::read_to_string;
    use std::path::PathBuf;

    fn test_path(name: &str) -> PathBuf {
        current_dir().unwrap().join("test_is_there").join(name)
    }

    fn test_files(
        input_name: &str,
        output_name: &str,
        fun: fn(&[Segment], &[Query]) -> Vec<Option<bool>>,
    ) {
        let (input_path, output_path) = (test_path(input_name), test_path(output_name));
        let (input, output) = (
            read_to_string(input_path).unwrap(),
            read_to_string(output_path).unwrap(),
        );

        let (arr, queries) = parse_input(&input);
        let expected: Vec<Option<bool>> = parse_output(&output);
        assert_eq!(fun(&arr, &queries), expected);
    }

    #[test]
    fn test_construction_basic_ops() {
        let t = SegmentTreeHash::from_slice(&[2, 4, 3, 2, 1]);
        assert!(t.is_there(0, 4, 4).is_ok_and(|v| v));
        assert!(t.is_there(0, 4, 0).is_ok_and(|v| !v));
        assert!(t.is_there(1, 3, 1).is_ok_and(|v| !v));
        assert!(t.is_there(1, 4, 1).is_ok_and(|v| v));
    }

    #[test]
    fn test_io_segment_tree() {
        for i in 0..=7 {
            let input = format!("input{i}.txt");
            let output = format!("output{i}.txt");
            test_files(&input, &output, solve_segment_tree);
        }
    }

    #[test]
    fn test_io_binary_lookup() {
        for i in 0..=7 {
            let input = format!("input{i}.txt");
            let output = format!("output{i}.txt");
            test_files(&input, &output, solve_binary_lookup);
        }
    }
}
