#![allow(unused_imports)]
use code::set6::longest_k_good_segment::{longest_k_good_segment};

/**
 * Since there can be more multiple longest_k_good we consider a test passed
 * if the lenght of any longest_k_good matches the length of the testcases:
 * 
 * TestCases got from: https://codeforces.com/contest/616/problem/D?locale=en
 */

#[test]
fn test_1() -> (){
	let v = [1,2,3,4,5];
	let k = 5;
	
	let (_,_,diff) = longest_k_good_segment(&v, k).unwrap();
	assert_eq!(diff + 1,5);
}

#[test]
fn test_2() -> (){
	let v = [6,5,1,2,3,2,1,4,5];
	let k = 3;
	
	let (_,_,diff) = longest_k_good_segment(&v, k).unwrap();
	assert_eq!(diff + 1,5);
}

#[test]
fn test_3() -> (){
	let v = [1,2,3];
	let k = 1;
	
	let (_,_,diff) = longest_k_good_segment(&v, k).unwrap();
	assert_eq!(diff + 1,1);
}