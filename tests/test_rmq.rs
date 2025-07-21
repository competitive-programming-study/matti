#![allow(unused_imports)]
use code::data_structs::segment_tree::SegmentTree;
use code::optional::set9::rmq::rmq;
use code::test_case;

#[test]
fn test_codeforces() {
    let a = [1, 2, 3, 4];
    let q = [(3, 0, None), (3, 0, Some(-1)), (0, 1, None), (2, 1, None)];
    let expected = vec![1, 0, 0];
    test_case!(rmq, (&a, &q), expected);
}
