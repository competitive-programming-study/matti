use std::cmp::Ordering::*;

/// FIND PEAK
/// 
/// Given an unsorted array of positive and negative integers find any peak item
/// if present. An item is a peak if the neighbours are strictly lower in value than
/// it.
/// 
/// Returns an `Option<i32>` which is `None` if the vector is empty
/// 
/// We can assume that leftmost and rightmost items are peaks if their corrisponding neighbour
/// is lower than them.
/// 
/// Since we have no restriction on which peak to return if multiple are present
/// we can base an implementation off of a discrete gradient descent, so comparing
/// a point (middle item) with it's neighbour, at every step we exclude half of the 
/// array (the one that doesn't point to the "gradient")
/// 
/// Given an item `n`, and its neighbours: `l_n`,`r_n`:
/// - `l_n < n && r_n < n`: it's a peak
/// - `l_n < n < r_n `: we exclude the left side of the array (we're going up)
/// - `l_n > n > r_n`: we exclude the right side of the array (we're going down)
/// - `l_n > n && r_n > n`: expect to find a peak on both sides (since one must exist
/// or the leftmost or rightmost are), so we can choose where to move (we'll move left)
/// 
/// **IMPORTANT**: basing the approach over gradient descent, we produce a non-deterministic
/// output, since given an arbitrary set of points we could get stuck on a local maxima. (We can't
/// find a way to return the first or last peak without scanning the whole array in O(n))
/// 
/// *Time Complexity*: O(log_2(n))
/// 
/// *Space Complexity*: O(1)
/// 
/// 
pub fn peak(nums: &[i32]) -> Option<i32> {
    if nums.is_empty() {
        return None;
    };
    let (mut start, mut end) = (0usize, nums.len() - 1);

    //breaks when array is less than 2 elements
    while start < end {
        let mid = (start + end) / 2;
        let (lv, mv, rv) = (nums[mid - 1], nums[mid], nums[mid + 1]);
        
        match (lv.cmp(&mv),rv.cmp(&mv)) {
            (Less,Less) => {
                return Some(mv);
            }
            (Less,Greater) => {
                start = mid + 1
            }
            (Greater,Less) => {
                end = mid - 1
            }
            (Greater,Greater)|_ => {  //we can choose to move left or right (they both contain peaks)
                end = mid - 1
            }
        }
    };

    Some(nums[start].max(nums[end]))

}
