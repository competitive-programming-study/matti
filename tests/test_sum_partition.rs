#![allow(unused_imports)]
use code::optional::set12::sum_partition::sum_partition;

#[test]
fn test_empty_set() {
    let input = vec![];
    assert!(sum_partition(&input)); // Empty set → two empty subsets with sum 0
}

#[test]
fn test_single_element() {
    assert!(!sum_partition(&[1]));
    assert!(!sum_partition(&[100]));
}

#[test]
fn test_even_sum_partitionable() {
    assert!(sum_partition(&[1, 5, 11, 5])); // 11+1 = 5+5
    assert!(sum_partition(&[3, 1, 5, 9, 12])); // 12+3=15, 5+9+1=15
}

#[test]
fn test_even_sum_not_partitionable() {
    assert!(!sum_partition(&[1, 2, 3, 5])); // Total = 11, not divisible
    assert!(!sum_partition(&[2, 2, 3, 5])); // Total = 12, but no valid split
}

#[test]
fn test_all_same_elements_even_count() {
    assert!(sum_partition(&[4, 4, 4, 4])); // Two subsets of [4,4]
}

#[test]
fn test_all_same_elements_odd_count() {
    assert!(!sum_partition(&[4, 4, 4])); // Total = 12, but not splittable equally
}

#[test]
fn test_large_input_partitionable() {
    let input = vec![1; 100]; // Total = 100 → can be split into two sets of 50
    assert!(sum_partition(&input));
}

#[test]
fn test_large_input_not_partitionable() {
    let mut input = vec![1; 99];
    input.push(2); // Total = 101 → odd → can't be split
    assert!(!sum_partition(&input));
}
