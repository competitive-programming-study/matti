use std::cmp::Ordering::*;

///
/// **FIND MIN IN ROTATED SORTED ARRAY**
/// 
/// Given a sorted array which is been rotated of an unknown number of times return
/// the minimum value
/// 
/// Returns an `Option<i32>` which is `None` if the vector was empty
/// 
/// The items should be unique, but we can use an hack to get the general case.
/// 
/// By intuition, we can compare the item in the middle, with the item at the end (same thing
/// could have been achieved comparing with the start ).
/// 
/// If the middle is greater than the end, the array has been rotated (right)
/// of at least (middle positions), so the minimum will be in the right side
/// 
/// Else the middle is in the leftmost part of the array.
/// 
/// To account for duplicate elements, we know that if the the element in the middle is 
/// equal to the element at the end, then all the elements in between must be equal to those
/// (since the array is sorted), so we just exclude them.
/// 
/// *Time Complexity*: O(log_2(n))
/// 
/// *Space Complexity*: O(1)
///  
pub fn find_min(nums: &[i32]) -> Option<i32> {
    if nums.is_empty() {
        return None;
    }
    let (mut start, mut end) = (0usize, nums.len() - 1);
    while start < end {
        let mid = (start + end) / 2;
        match nums[mid].cmp(&nums[end]) {
            Greater => start = mid + 1,
            Less|Equal => end = mid
        }
    }
    Some(nums[start])
}

