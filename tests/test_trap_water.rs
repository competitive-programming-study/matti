#![allow(unused_imports)]
use code::set2::trap_water::{trap_water_2_pass,trap_water};
use code::test_util::TestCase;

type TestC<'a> = TestCase<&'a[u32],u32>;

const TO_TEST : [fn(&[u32])->u32;2] = [
    trap_water_2_pass,
    trap_water
];

#[test]
fn test_empty() -> () {
    TestC::new(&[0u32],0).test_multiple(&TO_TEST);
}

#[test]
fn test_1() -> () {
    let heights: Vec<u32> = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    TestC::new(&heights,6).test_multiple(&TO_TEST);
}

#[test]
fn test_2() -> () {
    let heights: Vec<u32> = vec![4,2,0,3,2,5];
    TestC::new(&heights,9).test_multiple(&TO_TEST);
}
