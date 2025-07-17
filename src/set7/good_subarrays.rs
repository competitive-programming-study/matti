use std::collections::HashMap;

///
/// ## Good subarrays
/// 
/// We're given a list of integers from 0 to 9 inclusive. We define 
/// a good subarray a list of integers where the sum of the integers
/// equals the length of the subarray
/// 
/// ### Returns
/// The numbers of subarrays
/// 
/// ### Strategy
/// If we calculated the prefix-sum array we would want all subarrays such that
/// sum[r] - sub[l] = r - l + 1;
/// 
/// We can reinstate this as
/// sub[l-1] - (l - 1) = sum[r] - r
/// 
/// so we can index the prefix sum as sum[i] - i
/// 
/// Then we can use a hashmap to accumulate the frequencies of such sums
/// and for each new one added we look in the map if we have that frequency
/// accumulating it, then we a new frequency to the map;
/// 
pub fn good_subarrays(nums: &[i64]) -> usize {
    let (mut count,mut sum) = (0,0);
    let mut map = HashMap::new();
    map.insert(-1,1); //we want to explictly add subarrays that start at index 0

    for (i,&e) in nums.iter().enumerate() {
        sum += e;
        let key = sum - (i as i64) -1;
        if let Some(&v) = map.get(&(key)) {
            count += v;
        }
        *map.entry(key).or_insert(0)+=1;
    }

    count
}