/// # Counting Inversions
///
/// Given a list of numbers count how many times, given 2 indexes i<j a[i] > a[j]
///
/// ## Naive Approach
/// Use 2 nested loops (i in 0,n) and (j in i+1, n) and accumulate the counts
///
/// ## Complexity
/// - **Time Complexity**: O(n^2)
/// - **Space Complexity**: O(1)
pub fn count_inversions_naive(nums: &[i64]) -> usize {
    let mut count = 0;

    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] > nums[j] {
                count += 1;
            }
        }
    }
    count
}

/// # Counting Inversions
///
/// Given a list of numbers count how many times, given 2 indexes i<j a[i] > a[j] is an inversion
///
/// # Returns all the inversions in the array
///
/// ## Mergesort Approach
/// Recursively divide the list, and when merging, if selecting from the right side
/// accumulate the left items from the left side
///
/// ## Complexity
/// - **Time Complexity**: O(nlog(n))
/// - **Space Complexity**: O(n)
pub fn count_inversions_merge(nums: &[i64]) -> usize {
    fn merge_with_count(nums: &mut [i64], start: usize, middle: usize, end: usize) -> usize {
        let (a, b) = (
            nums[start..=middle].to_vec(),
            nums[middle + 1..=end].to_vec(),
        );

        let (mut i, mut j, mut count) = (0, 0, 0);
        for n in nums.iter_mut().take(end + 1).skip(start) {
            if i < a.len() && (j >= b.len() || a[i] <= b[j]) {
                *n = a[i];
                i += 1;
            } else {
                *n = b[j];
                count += a.len() - i; //accumulate to count all left items
                j += 1;
            }
        }
        count
    }
    fn mergesort_with_count(nums: &mut [i64], start: usize, end: usize) -> usize {
        if start == end {
            0
        } else {
            let mid = (start + end) / 2;
            let left = mergesort_with_count(nums, start, mid);
            let right = mergesort_with_count(nums, mid + 1, end);
            let merge = merge_with_count(nums, start, mid, end);
            left + right + merge
        }
    }
    if nums.is_empty() {
        0
    } else {
        mergesort_with_count(&mut nums.to_vec(), 0, nums.len() - 1)
    }
}

use crate::data_structs::fenwick_tree::FenwickTree;

/// # Count Inversions
/// Given a list of numbers and 2 indexes such that i<j a[i] > a[j] it's an inversion
///
/// ## Returns
/// all inversions in the list
///
/// ## Fenwick Tree Approach
///
/// A fenwick tree is a neat data structure really optimized to
/// compute dynamic prefix sums. We can use this data structure as a counting structure
///
/// To compute inversions we need a fenwick tree that can hold the appearance count of
/// all integers in the list [min,max] interval.
///
/// The idea is, to initialize all appearance to 0, then for every integer we encounter we
/// compute the sum of all counts of integers greater than it. (`range_sum(i+1,fen.len())`).
///
/// We accumulate that count, essentially recording for every integer we see, all the bigger
/// integers that came before.
///
/// ## Complexity
/// ### Time Complexity
/// Constructing the Fenwick Tree takes O(m) (where m is list.max - list.min). Then computing
/// the range sum takes Theta(log(m)) so for every integer we result in the same complexity
/// of the mergesort based approach of O(mlog(m)).
///
/// ### Space Complexity
/// Even tho the fenwick tree can be an implicit data structure we still need it to be
/// a deep copy of the list so we result in O(n)
///
pub fn count_inversions_fenwick(nums: &[i64]) -> usize {
    if nums.is_empty() {
        return 0;
    }
    let (mut min, mut max, mut count) = (i64::MAX, i64::MIN, 0);

    //get the min-max of the nums in order to optimize the tree size
    for &e in nums {
        min = min.min(e);
        max = max.max(e);
    }

    //the length of the fenwick tree is the inclusive range
    let ln = (max - min) as usize + 1;
    //to get the correct index of the tree based on the value we simply add min
    let mut ft = FenwickTree::with_len(ln, 0);

    for &e in nums {
        let ft_idx = (e - min) as usize;
        //add to count the range sum from next index to the end
        count += ft.range_sum(ft_idx + 1, ft.len() - 1).unwrap();
        let _ = ft.add(ft_idx, 1);
    }

    count
}
