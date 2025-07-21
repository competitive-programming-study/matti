#![allow(unused_imports)]
use code::optional::set1::array_leaders::{self, array_leaders};
use code::test_util::TestCase;

type TestC<'a> = TestCase<&'a [u32], Vec<u32>>;

#[test]
fn test_empty() {
    let v = vec![];
    TestC::new(&v, vec![]).test(array_leaders);
}

#[test]
fn test_1() {
    let v: Vec<u32> = vec![16, 17, 4, 3, 5, 2];
    TestCase::new(&v as &[u32], vec![17, 5, 2]).test(array_leaders);
}

#[test]
fn test_2() {
    let v: Vec<u32> = vec![10, 4, 2, 4, 1];
    TestCase::new(&v as &[u32], vec![10, 4, 4, 1]).test(array_leaders);
}

#[test]
fn test_3() {
    let v: Vec<u32> = vec![5, 10, 20, 40];
    TestCase::new(&v as &[u32], vec![40]).test(array_leaders);
}

#[test]
fn test_4() {
    let v: Vec<u32> = vec![30, 10, 10, 5];
    TestCase::new(&v as &[u32], vec![30, 10, 10, 5]).test(array_leaders);
}
