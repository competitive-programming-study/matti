#![allow(unused_imports)]
use code::set10::power_array::power_array;
use code::test_case;
use code::test_util::TestCase;

#[test]
fn test_codeforces() -> () {
    //this test case considers 1-indexing
    test_case!(
        power_array,
        (&[i64::MAX, 1, 2, 1], &[(1, 2), (1, 3)]),
        vec![3, 6]
    );
    //this test case considers 0-indexing
    test_case!(
        power_array,
        (&[1, 1, 2, 2, 1, 3, 1, 1], &[(2, 7), (1, 6), (2, 7)]),
        vec![20, 20, 20]
    );
}
