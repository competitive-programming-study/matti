#![allow(unused_imports)]
use code::set1::missing_integer::*;
use code::test_util::TestCase;

const TO_TEST: [fn(&[u32])->Option<u32>;4] = [
    missing_integer_gauss,
    missing_integer_mark,
    missing_integer_xor,
    missing_integer_swap
];

type TestC<'a> = TestCase<&'a[u32],Option<u32>>;

#[test]
fn test_empty() -> () {
    let v = vec![];
    TestC::new(&v,None).test_multiple(&TO_TEST);
}

#[test]
fn test_1() -> () {
    let v: Vec<u32> = vec![1,0];
    TestC::new(&v,Some(2)).test_multiple(&TO_TEST);
}

#[test]
fn test_2() -> () {
    let v: Vec<u32> = vec![3,0,1];
    TestC::new(&v,Some(2)).test_multiple(&TO_TEST);
}

#[test]
fn test_3() -> () {
    let v: Vec<u32> = vec![9,6,4,2,3,5,7,0,1];
    TestC::new(&v,Some(8)).test_multiple(&TO_TEST);
}
