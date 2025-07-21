#![allow(unused)]

///# Max Subarray (Bruteforce)
///
/// Given an array of `n` signed integers (`i32`) we have to return
/// the max sum of any sub-array
///
/// ## Returns
/// `Option<i32>` that is `None` if the vector was empty
///
/// ## Strategy
/// The bruteforce approach consistes in iterating on all possible sub-array
/// sums, updating the current max-sum
///
/// For every possible sub-array we iterate through it, computing the sum
///
/// ## Complexity
/// We require no additional space while this solution runs in O(n^2) time
pub fn max_subarray_bruteforce(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        None
    } else {
        let mut max_sum_sf = i32::MIN;
        for i in 0..arr.len() {
            for j in i..arr.len() {
                let mut local_sum: i32 = 0;
                for item in arr.iter().take(j + 1).skip(i) {
                    local_sum += item;
                }
                max_sum_sf = max_sum_sf.max(local_sum)
            }
        }

        Some(max_sum_sf)
    }
}

///# Max Subarray (Bruteforce)
///
/// Given an array of `n` signed integers (`i32`) we have to return
/// the max sum of any sub-array
///
/// ## Returns
/// `Option<i32>` that is `None` if the vector was empty
///
/// ## Strategy
/// Instead of iterating on each possible subarray sum, we iterate on all
/// possible subarrays, updating a local sum
///
/// For every possible sub-array we iterate through it, computing the sum
///
/// ## Complexity
/// We require no additional space while this solution runs in O(n^2) time
pub fn max_subarray_bruteforce_optimized(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        None
    } else {
        let mut max_sum_sf = i32::MIN;
        for i in 0..arr.len() {
            let mut local_sum = 0;
            for v in arr.iter().skip(i) {
                local_sum += *v;
                max_sum_sf = max_sum_sf.max(local_sum);
            }
        }
        Some(max_sum_sf)
    }
}

///# Max Subarray (Bruteforce)
///
/// Given an array of `n` signed integers (`i32`) we have to return
/// the max sum of any sub-array
///
/// ## Returns
/// `Option<i32>` that is `None` if the vector was empty
///
/// ## Strategy
/// We iterate on the array, by checking each item. For each subarray we can
/// maximize the sum in two ways:
/// - if the first item of the subarray is negative, exluding it achieves a bigger sum
/// - if current subarray sum is negative by, removing last element we can maximize it
///
/// ## Complexity
/// We require no additional space while this solution runs in O(n) time
pub fn max_subarray_kadane(nums: &[i32]) -> Option<i32> {
    if nums.is_empty() {
        None
    } else {
        let (mut local_sum, mut max_sf) = (0, i32::MIN);

        for &item in nums {
            if local_sum > 0 {
                //accumulate items until local sum is positive
                local_sum += item;
            } else {
                // reset sum if negative or null items
                local_sum = item;
            }

            //update max so far at each step
            max_sf = max_sf.max(local_sum);
        }
        Some(max_sf)
    }
}

#[cfg(test)]
mod test_subarray {
    use super::*;

    #[test]
    fn test_basic() {
        let input = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let expected_output = Some(6);

        assert_eq!(max_subarray_bruteforce(&input), expected_output);
        assert_eq!(max_subarray_bruteforce_optimized(&input), expected_output);
        assert_eq!(max_subarray_kadane(&input), expected_output);
    }
}
