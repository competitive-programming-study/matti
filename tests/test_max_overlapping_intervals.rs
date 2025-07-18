#![allow(unused_imports)]
use code::set6::max_overlapping_intervals::{max_overlapping_explicit, max_overlapping_ideomatic};
use code::test_util::TestCase;

type TestC<'a> = TestCase<&'a [(usize, usize)], usize>;

#[test]
fn test_empty() -> () {
    TestC::new(&vec![], 0).test(max_overlapping_ideomatic);
    TestC::new(&vec![], 0).test(max_overlapping_explicit);
}

#[test]
fn test_1() -> () {
    let v: Vec<(usize, usize)> = vec![
        (0, 2),
        (3, 4),
        (5, 8),
        (8, 9),
        (4, 7),
        (0, 3),
        (2, 5),
        (7, 9),
        (2, 4),
        (3, 5),
    ];
    TestC::new(&v, 5).test(max_overlapping_ideomatic);
    TestC::new(&v, 5).test(max_overlapping_explicit);
}
