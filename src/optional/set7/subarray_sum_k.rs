use std::collections::HashMap;

///
/// ## Subarray Sum k
///
/// Given a list of integers, and an interger k return how many integers sum up to k
///
/// ### Strategy
///
/// If we precompute the prefix-sum array, for every sub-array we would ask queries as
/// `sum[j] - sum[i] = k` (the sum in the range is equal to k)
///
/// This is the same of asking `sum[i] = sum[j] - k`. So instead of precomputing the whole
/// array we accumulate the sum of the items in a variable. For each accumulation we check
/// in a hashmap for the frequency of sum - k; we increment the count by the frequency.
///
/// After that we insert the new sum in the map (incrementing its frequency or setting it
/// to 1)
///
/// ### Complexity
/// Since we sweep the array only one time, and the query to the hashmap have constant time
/// complexity it's O(n) in time. It's O(n) in space at the worst case when all sums are unique
pub fn subarray_sum(nums: &[i64], k: i64) -> usize {
    let (mut count, mut sum) = (0, 0);

    let mut map = HashMap::new();
    map.insert(0, 1); //we account for subarrays that start at index 0
    for &v in nums {
        sum += v;

        //checking if sum - k is in the map is the same as asking
        //Have we seen a sum[i] such that sum - sum[i] = k?
        if let Some(&freq) = map.get(&(sum - k)) {
            count += freq;
        }

        //we update the frequency count of the current seen sum
        *map.entry(sum).or_insert(0) += 1;
    }

    count
}
