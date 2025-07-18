#![allow(unused_imports)]
use code::set8::counting_inversions::{
    count_inversions_fenwick, count_inversions_merge, count_inversions_naive,
};
use code::test_util::TestCase;

type TestC<'a> = TestCase<&'a [i64], usize>;

const TO_TEST: [fn(&[i64]) -> usize; 3] = [
    count_inversions_fenwick,
    count_inversions_merge,
    count_inversions_naive,
];

#[test]
fn test_empty() -> () {
    TestC::new(&[], 0).test_multiple(&TO_TEST);
}

#[test]
fn test_none() -> () {
    TestC::new(&[1, 2], 0).test_multiple(&TO_TEST);
    TestC::new(&[1, 2, 3, 4, 5, 5, 5, 5, 5, 7], 0).test_multiple(&TO_TEST);
    TestC::new(&[-1, 0, 1, 2], 0).test_multiple(&TO_TEST);
}

#[test]
fn test_one() -> () {
    TestC::new(&[2, 1], 1).test_multiple(&TO_TEST);
    TestC::new(&[2, -1], 1).test_multiple(&TO_TEST);
}

#[test]
fn test_two() -> () {
    TestC::new(&[2, 1, 1], 2).test_multiple(&TO_TEST);
    TestC::new(&[2, -1, 1], 2).test_multiple(&TO_TEST);
}

#[test]
fn test_some() -> () {
    TestC::new(&[3, 5, 1, 10, 9, 2, 6, 8], 11).test_multiple(&TO_TEST);
    TestC::new(&[4, 3, 2, 1, 0, -1], 15).test_multiple(&TO_TEST);
}
