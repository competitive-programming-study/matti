#![allow(unused_imports)]
use code::set3::find_peak::peak;
use code::test_util::{TestCase};

type TestC<'a> = TestCase<&'a[i32],Option<i32>>;

#[test]
fn test_empty() -> (){
	let a: [i32;0] = [];
	TestC::new(&a,None).test(peak);
}

#[test]
fn test_leftmost() -> () {
	let a = [3,2,1];
	TestC::new(&a,Some(3)).test(peak);
}

#[test]
fn test_rightmost() -> () {
	let a = [1,2,3];
	TestC::new(&a,Some(3)).test(peak);
}

#[test]
fn test_middle() -> () {
	let a = [1,2,3,2,1];
	TestC::new(&a,Some(3)).test(peak);
}

#[test]
fn test_multiple() -> () {
	let a = [3,2,3,2,3,2,3];
	TestC::new(&a,Some(3)).test(peak);
}