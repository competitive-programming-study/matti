#![allow(unused_imports)]
use code::set7::subarray_mod_sum_k::good_subarray;
use code::test_case;
use code::test_util::TestCase;

#[test]
fn test_false() -> () {
    for i in -5..5 {
        test_case!(good_subarray, (&[], i), false);
        test_case!(good_subarray, (&[i], i), false);
    }
}

#[test]
fn test_some() -> () {
    test_case!(good_subarray, (&[23, 2, 4, 6, 7], 6), true);
    test_case!(good_subarray, (&[23, 2, 6, 4, 7], 6), true);
    test_case!(good_subarray, (&[23, 2, 6, 4, 7], 13), false);
}
