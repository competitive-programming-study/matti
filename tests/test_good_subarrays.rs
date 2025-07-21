#![allow(unused_imports)]
use code::optional::set7::good_subarrays::good_subarrays;
use code::test_case;
use code::test_util::TestCase;

#[test]
fn test_empty() {
    test_case!(good_subarrays, (&[]), 0);
}

#[test]
fn test_one() {
    test_case!(good_subarrays, (&[0]), 1);
    test_case!(good_subarrays, (&[1, 1, 0]), 2);
    test_case!(good_subarrays, (&[1, 0, 2]), 2);
}
