#![allow(unused_imports)]
use code::optional::set11::min_jumps::*;
use code::test_case;

fn run_both(slice: &[usize], expected: Option<usize>) {
    test_case!(min_jumps, (slice), expected);
    test_case!(min_jumps_greedy, (slice), expected);
}

#[test]
fn test_empty_or_single() {
    run_both(&[], None);
    run_both(&[1], Some(1));
}

#[test]
fn test_cannot_jump() {
    run_both(&[1, 0, 0], None);
    run_both(&[0, 2], None);
}

#[test]
fn test_exact_jumps() {
    run_both(&[2, 3, 1, 1, 4], Some(2)); // 0→1→4
    run_both(&[1, 1, 1, 1], Some(3)); // 0→1→2→3
    run_both(&[3, 2, 1, 0, 4], None); // Can't reach end
}

#[test]
fn test_large_jump() {
    run_both(&[5, 1, 1, 1, 1], Some(1)); // one jump covers all
    run_both(&[4, 2, 0, 0, 2, 0], Some(2)); // 0 -> 4 -> out
}

#[test]
fn test_alternating() {
    run_both(&[1, 3, 0, 1, 4], Some(2)); // 0 -> 1 -> 4
}

#[test]
fn test_big_input() {
    let mut long_jump = vec![1; 1000];
    long_jump[0] = 999;
    run_both(&long_jump, Some(1));
}
