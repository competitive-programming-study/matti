#![allow(unused_imports)]
use code::optional::set15::woodcutters::woodcutters;
use code::test_util::TestCase;

#[test]
fn test_empty() {
    let trees: &[(i64, usize)] = &[];
    assert_eq!(woodcutters(trees), 0);
}

#[test]
fn test_single_tree() {
    let trees = &[(0, 5)];
    assert_eq!(woodcutters(trees), 1);
}

#[test]
fn test_multiple_trees() {
    let trees = &[(1, 3), (5, 2), (8, 4)];
    assert_eq!(woodcutters(trees), 3);
}

#[test]
fn test_no_overlap() {
    let trees = &[(1, 3), (5, 2), (9, 4)];
    assert_eq!(woodcutters(trees), 3);
}

#[test]
fn test_some_overlap() {
    let trees = &[(1, 3), (4, 2), (6, 5), (9, 1)];
    assert_eq!(woodcutters(trees), 3);
}

#[test]
fn test_all_trees_cut() {
    let trees = &[(1, 1), (5, 1), (10, 1)];
    assert_eq!(woodcutters(trees), 3);
}

#[test]
fn test_no_cut() {
    let trees = &[(1, 2), (2, 2), (6, 3)];
    assert_eq!(woodcutters(trees), 3);
}