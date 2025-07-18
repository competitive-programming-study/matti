#![allow(unused_imports)]
use code::set5::frogs_mosquitos::frog_mosquitos;
use code::test_case;

//Testcases from [[https://codeforces.com/contest/609/problem/F?locale=en]]

#[test]
fn test_1() -> () {
    let frogs: Vec<(i32, i32)> = vec![(10, 2), (15, 0), (6, 1), (0, 1)];

    let mosquitos: Vec<(i32, i32)> = vec![(110, 10), (1, 1), (6, 0), (15, 10), (14, 100), (12, 2)];

    let expected: Vec<(usize, i32)> = vec![(3, 114), (1, 10), (1, 1), (1, 2)];

    test_case!(frog_mosquitos, (&frogs, &mosquitos), expected);
}

#[test]
fn test_2() -> () {
    let frogs: Vec<(i32, i32)> = vec![(10, 2)];

    let mosquitos: Vec<(i32, i32)> = vec![(20, 2), (12, 1)];

    let expected: Vec<(usize, i32)> = vec![(1, 3)];

    test_case!(frog_mosquitos, (&frogs, &mosquitos), expected);
}
