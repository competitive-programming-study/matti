use std::collections::HashMap;

///
/// # Continous subarray sum
///
/// Given a list of integers a good subarray is defined as:
/// 1. length >= 2
/// 2. sum of the items is a multiple of a given k
///
/// ## Returns
/// Returns true if the array is a good subarray false otherwise
///
/// ## Strategy
///
/// If we computed the prefix sum array for each index we could ask queries like
/// (sum[j] - sum[i]) mod k == 0?
///
/// since sum[j] - sum[i] mod k = 0 then they are congruent so in mod k we can say
/// sum[j] mod k = sum[i] mod k
///
/// We can use the hashmap trick and store in a hashmap the earliest index where we seen
/// that modulo appear (since good subarrays have a length >= 2). When looking for the modulo
/// we check if the index stored in the map is at least 2 previous than the current one
///
/// ## Complexity
/// O(n) since we iterate on the array only one time and hashmap have constant time complexity.
/// We could also reason about the fact that hashMaps can hae linear runtime due to collision.
/// We can swap them by employing a lookup table. (We choose not to do it for the sake of simplicity)
///
/// O(k) in space since we store as keys the modulos of k, but since k is a constant it's O(1)
///
pub fn continous_subarray_sum(nums: &[i64], k: i64) -> bool {
    if k == 0 {
        //all numbers multiplied by 0 are multiples of 0
        return nums.len() >= 2;
    }

    let mut map = HashMap::new();
    map.insert(0, -1); //we have to account for subarrays that start at 0

    let mut sum = 0;
    for (i, &e) in nums.iter().enumerate() {
        sum += e;

        let modulo = sum % k;

        if let Some(prev_index) = map.get(&(modulo)) {
            if i as i32 - prev_index >= 2 {
                return true;
            }
        }

        map.insert(modulo, i as i32);
    }

    false
}

#[cfg(test)]
mod test_continous_subarray_sum {
    use super::*;
    #[test]
    fn test_false() {
        for i in -5..5 {
            assert_eq!(continous_subarray_sum(&[], i), false);
            assert_eq!(continous_subarray_sum(&[i], i), false);
        }
    }

    #[test]
    fn test_some() {
        assert_eq!(continous_subarray_sum(&[23, 2, 4, 6, 7], 6), true);
        assert_eq!(continous_subarray_sum(&[23, 2, 6, 4, 7], 6), true);
        assert_eq!(continous_subarray_sum(&[23, 2, 6, 4, 7], 13), false);
    }
}
