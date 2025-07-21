#[allow(unused)]

///**MAX_SUBARRAY (bruteforce)**
///
/// Given an array of n signed integers (`i32`) we have to return
/// the max sum of any sub-array
/// @returns: `Option<i32>` that is `None` if the vector was empty
///
/// The bruteforce approach consistes in iterating on all possible sub-array's
/// sums, updating the current max-sum
///
/// For every possible sub-array we iterate through it, computing the sum
///
///
/// *Space Complexity:*   O(1)
///
/// *Time Complexity:*    O(n^3)
pub fn max_subarray_bruteforce(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }

    let mut sum = i32::MIN;

    for i in 0..arr.len() {
        for j in i..arr.len() {
            let mut local_sum: i32 = 0;
            for item in arr.iter().take(j + 1).skip(i) {
                local_sum += item;
            }
            if local_sum > sum {
                sum = local_sum;
            }
        }
    }

    Some(sum)
}

///**MAX_SUBARRAY (bruteforce optimized)**
///
/// Given an array of n signed integers (`i32`) we have to return
/// the max sum of any sub-array
/// @returns: `Option<i32>` that is `None` if the vector was empty
///
/// Consists in iterating on all possible sub-arrays, updating a local sum
/// variable, checking if it's the biggest sum so far
///
/// *Space Complexity:*   O(1)
///
/// *Time Complexity:*    O(n^2)
pub fn max_subarray_bruteforce_optimized(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }
    let mut sum = i32::MIN;
    for i in 0..arr.len() {
        let mut local_sum = 0;
        for v in arr.iter().skip(i) {
            local_sum += *v;
            if local_sum > sum {
                sum = local_sum;
            }
        }
    }
    Some(sum)
}

///**MAX_SUBARRAY (Kadane's algorithm)**
///
/// Given an array of n signed integers (`i32`) we have to return
/// the max sum of any sub-array
/// returns: `Option<i32>` that is `None` if the vector was empty
///
/// This approach works on some invariants:
/// 1. given a sub-array which starts with a negative number, we can achieve
///    a subarray with a bigger sum not counting the first element
/// 2. given a all-negative integers array, the max subarray only contains the
///    max (negative) integer
///
/// *Space Complexity:* O(1)
///
/// *Time Complexity:* O(n)
///
pub fn kadane(nums: &[i32]) -> Option<i32> {
    if nums.is_empty() {
        return None;
    }
    let (mut sum, mut max) = (0, i32::MIN);

    for &item in nums {
        if sum > 0 {
            //If this is true we were only accumulating positive items
            sum += item; //add the item
        } else {
            sum = item; //reset sum if accumulating negatives
        }

        if sum > max {
            //commit sum to max if it's more than the previous max
            max = sum;
        }
    }

    Some(max)
}

/// MAX SUBARRAY WITH RANGE
///
/// Follow-up on kadane's algorithm, it returns an `Option` (which is `None` if
/// the array is empty).
///
///
/// returns `Option<(i32 ,usize,usize)>` with the max sum, and respectively start and end indexes
///
/// *Complexity: same as Kadane's*
pub fn kadane_w_range(nums: &[i32]) -> Option<(i32, usize, usize)> {
    if nums.is_empty() {
        return None;
    }

    let (mut sum, mut max) = (0, i32::MIN);
    let mut left = 0;
    let (mut max_left, mut max_right) = (0, 0);

    for (i, &v) in nums.iter().enumerate() {
        if sum > 0 {
            sum += v;
        } else {
            sum = v;
            left = i; //we reset the left index if resetting sum
        }

        if sum > max {
            max = sum;
            //we commit the indexes when committing sum
            max_left = left;
            max_right = i;
        }
    }

    Some((max, max_left, max_right))
}
