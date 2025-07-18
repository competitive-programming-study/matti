#![allow(unused_imports)]
use code::set3::find_min_rotated::find_min;
use code::test_util::TestCase;

type TestC<'a> = TestCase<&'a [i32], Option<i32>>;

#[test]
fn test_empty() -> () {
    let a: [i32; 0] = [];
    TestC::new(&a, None);
}

#[test]
fn test_sorted() -> () {
    let a = [1, 2, 3, 4, 5];
    TestC::new(&a, Some(1));
}

#[test]
fn test_shifted_left_1() -> () {
    let a = [2, 3, 4, 5, 1];
    TestC::new(&a, Some(1));
}

#[test]
fn test_shifted_right_1() -> () {
    let a = [5, 1, 2, 3, 4];
    TestC::new(&a, Some(1));
}

#[test]
fn test_duplicate_1() -> () {
    let a = [5, 5, 5, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4];
    TestC::new(&a, Some(1));
}

#[test]
fn test_duplicate_2() -> () {
    let a = [5, 5, 5, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4];
    TestC::new(&a, Some(1));
}
