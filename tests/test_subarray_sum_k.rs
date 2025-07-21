#![allow(unused_imports)]
use code::optional::set7::subarray_sum_k::subarray_sum;
use code::test_case;
use code::test_util::TestCase;

#[test]
fn test_empty() {
    for i in -5..5 {
        test_case!(subarray_sum, (&[], i), 0);
    }
}

#[test]
fn test_full() {
    let a = [0, 0, 0, 0, 0, 0]; //we have to consider all possible subarrays of different length
    test_case!(subarray_sum, (&a, 0), 21);
}

#[test]
fn test_one() {
    let a = [0, 1, 1, 0, 1, 1, 0]; //we only want subarrays that sum up to 1
    test_case!(subarray_sum, (&a, 1), 8);
}

#[test]
fn test_some() {
    test_case!(subarray_sum, (&[3, 0, 3], 3), 4);
    test_case!(subarray_sum, (&[1, 2, 3], 3), 2);
    test_case!(subarray_sum, (&[3, 0, -3], 0), 2);
}
