#![allow(unused_imports)]
use code::optional::set6::closest_points::{closest_pair, closest_pair_bruteforce};
use code::test_case;

#[test]
fn test_empty() {
    let points: Vec<(i64, i64)> = vec![];
    test_case!(closest_pair_bruteforce, (&points), None);
    test_case!(closest_pair, (&points), None);
}

#[test]
fn test_one_point() {
    let points = vec![(1, 2)];
    test_case!(closest_pair_bruteforce, (&points), None);
    test_case!(closest_pair, (&points), None);
}

#[test]
fn test_two_points() {
    let points = vec![(0, 0), (3, 4)];
    // distance squared = 3^2 + 4^2 = 25
    test_case!(closest_pair_bruteforce, (&points), Some(25));
    test_case!(closest_pair, (&points), Some(25));
}

#[test]
fn test_three_points() {
    let points = vec![(0, 0), (5, 12), (3, 4)];
    // (0,0) to (3,4) is the closest -> 25
    test_case!(closest_pair_bruteforce, (&points), Some(25));
    test_case!(closest_pair, (&points), Some(25));
}

#[test]
fn test_duplicate_points() {
    let points = vec![(1, 1), (1, 1), (2, 2)];
    // duplicate points -> distance = 0
    test_case!(closest_pair_bruteforce, (&points), Some(0));
    test_case!(closest_pair, (&points), Some(0));
}

#[test]
fn test_random_points() {
    let points = vec![(10, 10), (20, 20), (15, 15), (30, 30), (12, 11)];
    let expected = closest_pair_bruteforce(&points);
    let result = closest_pair(&points);
    assert_eq!(result, expected);
}
