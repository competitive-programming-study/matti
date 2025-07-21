use std::collections::HashMap;

/// # Longest K-Good Segment
/// Given an array `nums` of `n` integers (with duplicates) and a parameter `k: usize > 0`
/// return the longest k-good segment, so the longest subarray with no more than `k` distinct
/// values
///
/// ## Returns
/// Returns an `Option<(usize,usize)>` with the start and end index of the subarray
///
/// ## Strategy
/// Use a two-pointer approach scanning each window of the array. Keep a counter (that tracks distinct)
/// values and a HashMap for frequency counting.
///
/// 1. init the map
/// 2. init 2 pointers (left,right) and their respective global best at 0
/// 3. insert a the number pointer by right, or if present increment it by one
/// 4. If map.length >= k we have to evict one number, so we decrement the number pointed by left (if 0 we remove it and we're done)
/// 5. check if the differenct between right and left is greater than actual best
///
/// ## Complexity
/// We take linear time to process the array. In order to keep the Hash Map,
/// we consider the case where the insertion of each element generates a collision,
/// this would take O(n) steps for each element. Insert we could use a lookup vector
/// with coordinate normalization, to store the frequency map, to beneficiate of
/// constant time lookup.
///
/// With this approach we take linear O(n) time to process the solution.
///
/// We need O(k) additional space to store the frequencies of each different item
///
pub fn longest_k_good_segment(nums: &[i64], k: usize) -> Option<(usize, usize, usize)> {
    //Early checks
    if nums.is_empty() || k == 0 {
        return None;
    } else if nums.len() < k {
        return Some((0, nums.len() - 1, nums.len()));
    }

    //init the required pointers
    let (mut left, mut best_left, mut best_right) = (0, 0, 0);
    let mut frequency: HashMap<i64, usize> = HashMap::new();

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

    Some((best_left, best_right, best_right - best_left))
}

#[cfg(test)]
mod test_longest_k_good {
    use super::*;

    #[test]
    fn test_1() {
        let v = [1, 2, 3, 4, 5];
        let k = 5;
        let (_, _, diff) = longest_k_good_segment(&v, k).unwrap();
        assert_eq!(diff, 4);
    }

    #[test]
    fn test_2() {
        let v = [6, 5, 1, 2, 3, 2, 1, 4, 5];
        let k = 3;

        let (_, _, diff) = longest_k_good_segment(&v, k).unwrap();
        assert_eq!(diff, 4);
    }

    #[test]
    fn test_3() {
        let v = [1, 2, 3];
        let k = 1;

        let (_, _, diff) = longest_k_good_segment(&v, k).unwrap();
        assert_eq!(diff, 0);
    }
}
