#![allow(unused_imports)]
use code::set2::max_sliding_window::*;
use code::test_case;
use code::test_util::*;

const TO_TEST: [fn(&[i32], usize) -> Option<Vec<i32>>; 5] = [
    max_sliding_bruteforce,
    max_sliding_ideomatic,
    max_sliding_window_bst,
    max_sliding_window_heap,
    max_sliding_window_deque,
];

#[test]
fn test_empty() -> () {
    let mut v: Vec<i32> = vec![]; //checks for empty vector
    for k in 0usize..3 {
        for f in TO_TEST {
            test_case!(f, (&v, k), None);
        }
    }

    v = vec![1, 2, 3, 4, 5, 6];
    for f in TO_TEST {
        test_case!(f, (&v, 0usize), None);
    }
}

#[test]
fn test_1() -> () {
    let v = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3usize;
    for f in TO_TEST {
        test_case!(f, (&v, k), Some(vec![3, 3, 5, 5, 6, 7]));
    }
}

#[test]
fn test_2() -> () {
    let v = vec![1];
    let k = 1usize;
    for f in TO_TEST {
        test_case!(f, (&v, k), Some(vec![1]));
    }
}
#[test]
fn test_3() -> () {
    let v = vec![7, 2, 5, 3, 4, 3];
    let k = 3usize;
    for f in TO_TEST {
        test_case!(f, (&v, k), Some(vec![7, 5, 5, 4]));
    }
}
