#![allow(unused_imports)]
use code::set2::next_larger_element::{self, next_larger_element};
use code::test_util::TestCase;

type TestC<'a> = TestCase<&'a [u32], Option<Vec<i32>>>;

#[test]
fn test_empty() -> () {
    TestC::new(&vec![], None).test(next_larger_element);
}

#[test]
fn test_1() -> () {
    TestC::new(&vec![7, 2, 5, 3, 4], Some(vec![-1, 5, -1, 4, -1])).test(next_larger_element);
}

#[test]
fn test_2() -> () {
    TestC::new(&vec![3, 2, 1], Some(vec![-1, -1, -1])).test(next_larger_element);
}

#[test]
fn test_3() -> () {
    TestC::new(&vec![1, 2, 3, 4, 5, 6, 7], Some(vec![2, 3, 4, 5, 6, 7, -1]))
        .test(next_larger_element);
}
