#![allow(unused_imports)]
use code::optional::set14::wilbur_array::wilbur_array;
use code::test_util::TestCase;

#[test]
fn test_empty() {
    let target: &[isize] = &[];
    assert_eq!(wilbur_array(target), 0);
}

#[test]
fn test_single_element() {
    let target = &[5];
    assert_eq!(wilbur_array(target), 5);
}

#[test]
fn test_multiple_elements() {
    let target = &[1, 4, 2, 8];
    assert_eq!(wilbur_array(target), 12);
}

#[test]
fn test_all_elements_same() {
    let target = &[3, 3, 3, 3];
    assert_eq!(wilbur_array(target), 3);
}

#[test]
fn test_negative_numbers() {
    let target = &[-3, -1, -7, -4];
    assert_eq!(wilbur_array(target), 14);
}
