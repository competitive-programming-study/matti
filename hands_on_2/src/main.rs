use hands_on_2::is_there::{self};
use hands_on_2::min_max::{self, Query::*};
use std::{fs, path::PathBuf};

///
/// Example on provided function in lib.rs
///
/// The test case is the same as test_min_max/input0.txt
///
fn min_max_example() {
    //we can provide the array and the queries
    let a = [1, 4, 2, 3, 4];
    let q = [
        Max((5, 5)),
        Max((5, 5)),
        Max((3, 4)),
        Max((1, 3)),
        Max((1, 4)),
    ];
    let out = min_max::solve(&a, &q);
    //we remap the expected output since solve returns an array of Options
    let expected_out: Vec<_> = vec![4, 4, 3, 4, 4].iter().map(|&x| Some(x)).collect();
    assert_eq!(out, expected_out);
}

///
/// Example on provided function in lib.rs
///
/// The test case is loaded from the file test_min_max/input0.txt
///
fn min_max_parsing_example() {
    let input_path = PathBuf::from("test_min_max")
        .join("input0")
        .with_extension("txt");
    let output_path = PathBuf::from("test_min_max")
        .join("output0")
        .with_extension("txt");

    let (a, q) = min_max::parse_input(&fs::read_to_string(&input_path).unwrap());
    // parse procedures remap the result to Option<_>
    let expected = min_max::parse_output(&fs::read_to_string(&output_path).unwrap());
    assert_eq!(min_max::solve(&a, &q), expected);
}

fn is_there_example() {
    //We can provide the array of segments and queries
    let a = [
        (2, 6),
        (3, 8),
        (4, 6),
        (1, 1),
        (5, 9),
        (6, 7),
        (8, 9),
        (0, 7),
        (1, 2),
        (2, 7),
    ];
    let q = [
        (1, 7, 8),
        (4, 6, 6),
        (7, 7, 6),
        (5, 9, 3),
        (7, 8, 1),
        (1, 2, 0),
        (3, 7, 0),
        (4, 8, 6),
        (6, 9, 8),
    ];
    let out_segment = is_there::solve_segment_tree(&a, &q);
    let out_binary = is_there::solve_binary_lookup(&a,&q);
    assert_eq!(out_segment,out_binary);
    //We remap the expected output since solve returns an array of options
    let expected: Vec<Option<bool>> = [0, 1, 0, 1, 0, 0, 0, 1, 0]
        .iter()
        .map(|&v| Some(v != 0))
        .collect();
    assert_eq!(out_segment,expected);
    assert_eq!(out_binary, expected);

}

fn is_there_parsing_example() {
    let input_path = PathBuf::from("test_is_there")
        .join("input0")
        .with_extension("txt");
    let output_path = PathBuf::from("test_is_there")
        .join("output0")
        .with_extension("txt");

    // parse procedures remap the result to Option<_>
    let (a, q) = is_there::parse_input(&fs::read_to_string(&input_path).unwrap());
    let expected = is_there::parse_output(&fs::read_to_string(&output_path).unwrap());
    assert_eq!(is_there::solve_segment_tree(&a, &q), expected);
    assert_eq!(is_there::solve_binary_lookup(&a,&q), expected);
}

fn main() {
    min_max_example();
    min_max_parsing_example();
    is_there_example();
    is_there_parsing_example();
}
