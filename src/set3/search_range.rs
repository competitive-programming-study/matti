use std::cmp::Ordering::*;

///**BINARY SEARCH (Leftmost)**
///
/// Given a sorted slice, returns the index of first appearance of a target.
///
/// Returns an `Option<usize>` which is `None` if the target doesn't appear
/// in the slice
///
/// *Time Complexity*: O(log(n))
///
/// *Space Complexity*: O(1)
pub(crate) fn binary_search_leftmost<T>(nums: &[T], target: &T) -> Option<usize>
where
    T: Eq + Ord,
{
    if nums.is_empty() {
        return None;
    }

    let (mut start, mut end) = (0usize, nums.len());
    while start < end {
        let mid = (start + end) / 2;
        match nums[mid].cmp(target) {
            Less => {
                start = mid + 1;
            }
            _ => {
                // Equal or Greater
                end = mid;
            }
        }
    }

    if start < nums.len() && &nums[start] == target {
        Some(start)
    } else {
        None
    }
}

///**BINARY SEARCH (Rightmost)**
///
/// Given a sorted slice, returns the index of last appearance of a target.
///
/// Returns an `Option<usize>` which is `None` if the target doesn't appear
/// in the slice
///
/// *Time Complexity*: O(log_2(n))
///
/// *Space Complexity*: O(1)
///
pub(crate) fn binary_search_rightmost<T>(nums: &[T], target: &T) -> Option<usize>
where
    T: Eq + Ord,
{
    if nums.is_empty() {
        return None;
    }

    let (mut start, mut end) = (0usize, nums.len());
    while start < end {
        let mid = (start + end) / 2;
        match nums[mid].cmp(target) {
            Greater => {
                end = mid;
            }
            _ => {
                // Equal or Greater
                start = mid + 1;
            }
        }
    }

    if nums[start - 1] == *target {
        Some(start - 1)
    } else {
        None
    }
}

/// SEARCH RANGE
///
/// Given a sorted slice find the first and last appearance of a target
///
/// Returns an `Option<(usize,usize)>` which is `None` if the target doesn't
/// appear in the slice
///
/// Use 2 binary searches specifically tuned to return the index of first and
/// last appearance
///
/// *Time Complexity*: 2 log_2(n) ~ O(log_2(n))
///
/// *Space Complexity*: O(1)
pub fn search_range<T>(nums: &[T], target: &T) -> Option<(usize, usize)>
where
    T: Ord + Eq,
{
    match binary_search_leftmost(nums, target) {
        None => None,
        Some(first) => Some((first, binary_search_rightmost(nums, target).unwrap())),
    }
}
