#![allow(unused_imports)]
use code::optional::set13::longest_increasing_subsequence::*;
use code::test_case;

fn run_both_versions(input: &[i32], expected: usize) {
    test_case!(longest_increasing_subsequence, (input), expected);
    test_case!(longest_increasing_subsequence_binary, (input), expected);
}

#[test]
fn test_empty() {
    run_both_versions(&[], 0);
}

#[test]
fn test_single_element() {
    run_both_versions(&[42], 1);
}

#[test]
fn test_strictly_increasing() {
    run_both_versions(&[1, 2, 3, 4, 5], 5);
}

#[test]
fn test_strictly_decreasing() {
    run_both_versions(&[5, 4, 3, 2, 1], 1);
}

#[test]
fn test_mixed_sequence() {
    run_both_versions(&[10, 9, 2, 5, 3, 7, 101, 18], 4); // [2,3,7,101]
    run_both_versions(&[0, 8, 4, 12, 2], 3); // [0,4,12]
}

#[test]
fn test_all_duplicates() {
    run_both_versions(&[7, 7, 7, 7], 1);
}

#[test]
fn test_zigzag_pattern() {
    run_both_versions(&[1, 3, 2, 4, 3, 5], 4); // [1,2,3,5]
}
