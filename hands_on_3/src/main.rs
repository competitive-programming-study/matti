use std::fs::read_to_string as fs_string;

use hands_on_3::course_design::{
    design_binary as course_design_binary, design_quadratic as course_design_quadratic,
    parse_input as parse_course_input, parse_output as parse_course_output,
};
use hands_on_3::holiday_planning::{
    parse_input as parse_holiday_input, parse_output as parse_holiday_output, plan as holiday_plan,
};

/**
 * The itinerary vectors must be concatenated into a flattened matrix
 */
fn holiday_planning_example() {
    let (cities, days) = (6, 8);
    let expected_max = 32;

    let itineraries = [
        3, 2, 1, 4, 2, 4, 3, 4, 3, 3, 1, 2, 3, 5, 5, 3, 3, 4, 1, 5, 3, 3, 4, 1, 3, 1, 5, 4, 3, 4,
        2, 5, 2, 5, 4, 4, 4, 5, 3, 4, 5, 1, 4, 4, 3, 2, 4, 5,
    ];

    assert_eq!(holiday_plan(&itineraries, cities, days), expected_max);
}

/**
 * Using test file `input0` and `output0` in folder `test_holiday`
 */
fn holiday_planning_parsing_example() {
    let test_folder = std::env::current_dir().unwrap().join("test_holiday");
    let (input, output) = (
        test_folder.join("input0.txt"),
        test_folder.join("output0.txt"),
    );

    let (cities, days, itineraries) = parse_holiday_input(&fs_string(&input).unwrap());
    let expected_output = parse_holiday_output(&fs_string(&output).unwrap());

    assert_eq!(holiday_plan(&itineraries, cities, days), expected_output);
}

fn course_desing_example() {
    let topics = [(0, 3), (99, 1), (11, 20), (1, 2), (10, 5)];
    let expected_output = 3;
    assert_eq!(course_design_quadratic(&topics), expected_output);
    assert_eq!(course_design_binary(&topics), expected_output);
}

/**
 * Using test file `input0` and `output0` in folder `test_design`
 */
fn course_design_parsing_example() {
    let test_folder = std::env::current_dir().unwrap().join("test_design");
    let (input, output) = (
        test_folder.join("input0.txt"),
        test_folder.join("output0.txt"),
    );

    let topics = parse_course_input(&fs_string(&input).unwrap());
    let expected_output = parse_course_output(&fs_string(&output).unwrap());

    assert_eq!(course_design_quadratic(&topics), expected_output);
    assert_eq!(course_design_binary(&topics), expected_output);
}

fn main() {
    holiday_planning_example();
    holiday_planning_parsing_example();
    course_desing_example();
    course_design_parsing_example();
}
