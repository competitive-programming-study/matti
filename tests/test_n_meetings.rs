#![allow(unused_imports)]
use code::optional::set14::n_meetings::*;

#[test]
fn test_empty() {
    let meetings = vec![];
    assert_eq!(n_meetings(&meetings), 0);
}

#[test]
fn test_single_meeting() {
    let meetings = vec![(1, 2)];
    assert_eq!(n_meetings(&meetings), 1);
}

#[test]
fn test_non_overlapping_meetings() {
    let meetings = vec![(1, 2), (3, 4), (5, 6)];
    assert_eq!(n_meetings(&meetings), 3);
}

#[test]
fn test_completely_overlapping_meetings() {
    let meetings = vec![(1, 5), (2, 6), (3, 7)];
    assert_eq!(n_meetings(&meetings), 1);
}

#[test]
fn test_partial_overlaps() {
    let meetings = vec![(1, 3), (2, 4), (3, 5), (6, 8)];
    // (1,3), (6,8) [ cant select (3,5) in the middle because first meeting ends at 3]
    assert_eq!(n_meetings(&meetings), 2);
}

#[test]
fn test_edge_case_equal_end_and_start() {
    let meetings = vec![(1, 3), (3, 5), (5, 6)];
    //select (1,3),(5,6)
    assert_eq!(n_meetings(&meetings), 2);
}

#[test]
fn test_multiple_valid_schedules() {
    let meetings = vec![(1, 4), (2, 3), (3, 5), (7, 9), (5, 8)];
    assert_eq!(n_meetings(&meetings), 2); // (2,3), (5,8) or (1,4), (5,8)
}

#[test]
fn test_unsorted_input() {
    let meetings = vec![(8, 9), (1, 2), (3, 4), (0, 6), (5, 7)];
    assert_eq!(n_meetings(&meetings), 4); // (1,2), (3,4), (5,7), (8,9)
}

#[test]
fn test_all_start_before_first_end() {
    let meetings = vec![(1, 10), (2, 3), (3, 4), (4, 5)];
    //select (2,3) and (4,5)
    assert_eq!(n_meetings(&meetings), 2);
}
