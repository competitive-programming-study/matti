#![allow(unused)]

/**
 * MAX SUBARRAY
 *
 * Given an array of n signed integers return the max sum of
 * a subarray
 */

/**
 * Brute force approach, iterate on all possible sub-arrays and
 * save the biggest sum.
 *
 * Runtime: O(n3)
 */
pub fn max_subarray_1(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }

    let mut sum: i32 = i32::MIN;

    for i in 0..arr.len() {
        for j in i..arr.len() {
            let mut local_sum: i32 = 0;
            for k in i..=j {
                local_sum += arr[k];
            }
            if local_sum > sum {
                sum = local_sum;
            }
        }
    }

    Some(sum)
}

/**
 * Still brute force approach but update the sum dynamically
 *
 * Runtime: O(n2)
 */
pub fn max_subarray_2(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }
    let mut sum: i32 = i32::MIN;
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

/**
 * Kadane's Algorithm
 *
 * Start with a global max sum inited to i32::MIN (signed) and
 * a local sum used as an accumulator initialized as 0
 *
 * if sum is 0 or less, then reset it:
 *  acts as initialization and negative items clearing
 * if sum is greater than 0 then accumulate to the next item
 *
 * Check if the sum is greater than the max: if so update it and continue
 *
 * Runtime: O(n)
 */
pub fn max_sub_array(nums: &[i32]) -> Option<i32> {
    if nums.is_empty() {
        return None;
    }
    let (mut sum, mut max) = (0, i32::MIN);

    for item in nums {
        if sum > 0 {
            //If this is true we were only accumulating positive items
            sum += *item; //add the item
        } else {
            sum = *item; //reset sum if accumulating negatives
        }

        if sum > max {
            //commit sum to max if it's more than the previous max
            max = sum;
        }
    }

    Some(max)
}

/**
 * Follow up: Kadane's with range
 *
 * Runtime of O(n) but returns the tuple of (max_sum, left_idx, right_idx)
 *
 * the idxs are the ones of the max contiguous subarray
 */
pub fn max_sub_array_w_range(nums: &[i32]) -> Option<(i32, usize, usize)> {
    if nums.is_empty() {
        return None;
    }

    let (mut sum, mut max) = (0, i32::MIN);
    let mut left = 0;
    let (mut max_left, mut max_right) = (0, 0);

    for (i, v) in nums.iter().enumerate() {
        if sum > 0 {
            sum += *v;
        } else {
            sum = *v;
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

#[cfg(test)]
mod test {
    use super::*;

    static TO_TEST: [fn(&[i32]) -> Option<i32>; 3] =
        [max_sub_array, max_subarray_1, max_subarray_2];

    struct TestCase {
        input: Vec<i32>,
        output: Option<i32>,
        output_w_range: Option<(i32, usize, usize)>,
    }

    impl TestCase {
        fn new(
            input: Vec<i32>,
            output: Option<i32>,
            output_w_range: Option<(i32, usize, usize)>,
        ) -> Self {
            TestCase {
                input,
                output,
                output_w_range,
            }
        }
    }

    #[test]
    fn test_all_implementations() {
        let test_cases = vec![
            TestCase::new(vec![], None, None),
            TestCase::new(
                vec![-2, 1, -3, 4, -1, 2, 1, -5, 4],
                Some(6),
                Some((6, 3, 6)),
            ),
            TestCase::new(vec![-1], Some(-1), Some((-1, 0, 0))),
            TestCase::new(vec![5, 4, -1, 7, 8], Some(23), Some((23, 0, 4))),
        ];

        for case in &test_cases {
            for func in TO_TEST {
                assert_eq!(
                    func(&case.input),
                    case.output,
                    "Failed on input {:?} with function {:?}",
                    case.input,
                    std::any::type_name_of_val(&func) // optional: shows the function name
                );
            }

            if let Some(expected_range) = case.output_w_range {
                assert_eq!(
                    max_sub_array_w_range(&case.input),
                    Some(expected_range),
                    "Failed on input {:?} with max_sub_array_w_range",
                    case.input
                );
            } else {
                assert_eq!(
                    max_sub_array_w_range(&case.input),
                    None,
                    "Expected None on input {:?} with max_sub_array_w_range",
                    case.input
                );
            }
        }
    }
}
