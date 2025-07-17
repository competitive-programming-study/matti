#![allow(unused_imports)]
use code::set8::nested_segments::{nested_segments};
use code::test_util::{TestCase};

type TestC<'a> = TestCase<&'a[(i32,i32)],Vec<usize>>;

#[test]
fn test_empty() -> (){
	TestC::new(&[],vec![]).test(nested_segments);
}
#[test]
fn test_none() -> () {
	TestC::new(&[(0,1),(2,3)],vec![0,0]).test(nested_segments);
}

#[test]
fn test_one() -> () {
	TestC::new(&[(0,1),(-1,2)],vec![0,1]).test(nested_segments);
}

#[test]
fn test_two() -> () {
	TestC::new(&[(-2,6),(-1,3),(4,5)],vec![2,0,0]).test(nested_segments);
}

#[test]
fn test_overlapping() -> () {
	TestC::new(&[(-1,1),(0,2)],vec![0,0]).test(nested_segments);
}

#[test]
fn test_multiple_inclusions() -> () {
	TestC::new(&[(-5,5),(-4,4),(-3,3),(-2,2)],vec![3,2,1,0]).test(nested_segments);
}


#[test]
fn test_codeforces() -> () {
	TestC::new(&[(1,8),(2,3),(4,7),(5,6)],vec![3,0,1,0]).test(nested_segments);
	TestC::new(&[(3,4),(1,5),(2,6)],vec![0,1,1]).test(nested_segments);
}




