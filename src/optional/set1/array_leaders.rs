#![allow(unused)]

///**Array Leaders**
///
/// Given an array of positive integers (`u32`) we have to find all the
/// leaders in the array
///
/// An element is considered a leader if it's greater or equal to all elements to its right
///
/// @par: arr: slice of `u32`
/// @returns a `Vec<u32>` with all the leaders
///
/// The the strategy is to iterate on the slice in reverse, keeping track of the current
/// max, pushing it to the leaders vector when it changes
///
/// Before returning the leaders vector, we reverse it.
///
/// Space Complexity:   O(1) (constant extra space required)
/// Time Complexity:    n + n/2 (for reverse in place) ~ O(n)
pub fn array_leaders(arr: &[u32]) -> Vec<u32> {
    if arr.is_empty() {
        return Vec::new();
    }

    let mut max = 0;
    let mut leaders = Vec::with_capacity(arr.len());

    for &e in arr.iter().rev() {
        if e >= max {
            max = e;
            leaders.push(max);
        };
    }

    leaders.reverse();
    leaders
}
