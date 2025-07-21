#![allow(unused_imports)]
use code::optional::set1::max_subarray::*;
use code::test_util::TestCase;

type TestC<'a> = TestCase<&'a [i32], Option<i32>>;
type TestCt<'a> = TestCase<&'a [i32], Option<(i32, usize, usize)>>;

const FUN: [fn(&[i32]) -> Option<i32>; 3] = [
    max_subarray_bruteforce,
    max_subarray_bruteforce_optimized,
    kadane,
];

#[test]
fn test_empty() {
    let v: Vec<i32> = vec![];
    TestC::new(&v, None).test_multiple(&FUN);

    TestCt::new(&v, None).test(kadane_w_range);
}

#[test]
fn test_1() {
    let v: Vec<i32> = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    TestC::new(&v, Some(6)).test_multiple(&FUN);

    TestCt::new(&v, Some((6, 3, 6))).test(kadane_w_range);
}
