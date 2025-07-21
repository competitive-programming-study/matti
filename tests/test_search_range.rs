#![allow(unused_imports)]
use code::optional::set3::search_range::search_range;
use code::test_case;

#[test]
fn test_empty() {
    test_case!(search_range, (&[], &10), None);
    test_case!(search_range, (&[], &20), None);
    test_case!(search_range, (&[], &30), None);
}

#[test]
fn test_no_target() {
    let v = [10, 20, 30, 40];
    test_case!(search_range, (&v, &11), None);
    test_case!(search_range, (&v, &21), None);
    test_case!(search_range, (&v, &31), None);
    test_case!(search_range, (&v, &41), None);
}

#[test]
fn test_one_target() {
    let v = [10, 20, 30, 40];
    test_case!(search_range, (&v, &10), Some((0usize, 0usize)));
    test_case!(search_range, (&v, &20), Some((1usize, 1usize)));
    test_case!(search_range, (&v, &30), Some((2usize, 2usize)));
    test_case!(search_range, (&v, &40), Some((3usize, 3usize)));
}
#[test]
fn test_multiple_targets() {
    let v = [1, 1, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    test_case!(search_range, (&v, &1), Some((0usize, 1usize)));
    test_case!(search_range, (&v, &2), Some((2usize, 4usize)));
    test_case!(search_range, (&v, &3), Some((5usize, 8usize)));
    test_case!(search_range, (&v, &4), Some((9usize, 12usize)));
}
