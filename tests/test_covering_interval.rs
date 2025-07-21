#![allow(unused_imports)]
use code::optional::set6::covering_interval::{covering_intervals, covering_intervals_sweep};
use code::test_case;
use code::test_util::TestCase;

#[test]
fn test_1() {
    let v: Vec<(i32, i32)> = vec![(1, 2), (3, 4), (5, 6)];

    let int = (2, 5);
    test_case!(covering_intervals, (&v, int), true);
    test_case!(covering_intervals_sweep, (&v, int), true);
}

#[test]
fn test_2() {
    let v: Vec<(i32, i32)> = vec![(1, 10), (10, 20)];

    let int = (21, 21);
    test_case!(covering_intervals, (&v, int), false);
    test_case!(covering_intervals_sweep, (&v, int), false);
}

#[test]
fn test_3() {
    let v: Vec<(i32, i32)> = vec![
        (8, 31),
        (2, 20),
        (22, 25),
        (13, 27),
        (7, 17),
        (17, 47),
        (14, 33),
        (28, 39),
    ];
    let int = (2, 15);
    test_case!(covering_intervals, (&v, int), true);
    test_case!(covering_intervals_sweep, (&v, int), true);
}
