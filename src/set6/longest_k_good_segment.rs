use std::collections::HashMap;

/// # Longest K-Good Segment
/// Given an array `nums` of `n` integers (with duplicates) and a parameter `k: usize > 0`
/// return the longest k-good segment, so the longest subarray with no more than `k` distinct
/// values
/// 
/// # Returns
/// Returns an `Option<(usize,usize)>` with the start and end index of the subarray
/// 
/// # Strategy
/// Use a two-pointer approach scanning each window of the array. Keep a counter (that tracks distinct)
/// values and a HashMap for frequency counting. 
/// 
/// 1. init the map
/// 2. init 2 pointers (left,right) and their respective global best at 0
/// 3. insert a the number pointer by right, or if present increment it by one
/// 4. If map.length >= k we have to evict one number, so we decrement the number pointed by left (if 0 we remove it and we're done)
/// 5. check if the differenct between right and left is greater than actual best
/// 
/// # Complexity
/// 
/// *Time Complexity*: O(n)
/// 
/// *Space Complexity*: O(k) ~ O(1) 
/// 
pub fn longest_k_good_segment(nums: &[i64], k: usize) -> Option<(usize,usize,usize)> {
    //Early checks
    if nums.is_empty() || k <= 0 {
        return None;
    } else if nums.len() < k {
        return Some((0,nums.len()-1,nums.len()));
    }

    //init the required poiinters
    let (mut left, mut best_left,mut best_right) = (0,0,0);
    let mut frequency: HashMap<i64,usize> = HashMap::new();

    for right in 0..nums.len() {
        *frequency.entry(nums[right]).or_insert(0) += 1;

        while frequency.len() > k {
            let count = frequency.get_mut(&nums[left])?;
            *count -= 1;
            if *count == 0 {
                frequency.remove(&nums[left]);
            }
            left += 1;
        }

        if (right - left) > (best_right - best_left) {
            best_left = left;
            best_right = right;
        }

    } 

    Some((best_left,best_right,best_right - best_left)) 
}