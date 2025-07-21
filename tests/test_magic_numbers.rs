#![allow(unused_imports)]
use code::optional::set14::magic_numbers::*;

#[test]
fn test_valid_magic_numbers() {
    assert!(magic_number(1));
    assert!(magic_number(14));
    assert!(magic_number(144));
    assert!(magic_number(141));
    assert!(magic_number(114));
    assert!(magic_number(1441));
    assert!(magic_number(14144));
}

#[test]
fn test_invalid_magic_numbers() {
    assert!(!magic_number(0));
    assert!(!magic_number(2));
    assert!(!magic_number(42));
    assert!(!magic_number(441));
    assert!(!magic_number(143));
    assert!(!magic_number(444));
    assert!(!magic_number(414));
}
