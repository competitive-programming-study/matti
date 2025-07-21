#![allow(unused_imports)]
use code::optional::set12::bool_knapsack::bool_knapsack;
use code::test_case;

#[test]
fn test_null_capacity() {
    let mut objects = vec![(10, 20), (1, 20), (1, 1), (2, 2)];
    test_case!(bool_knapsack, (&objects, 0), 0);
    objects.push((10, 0));
    test_case!(bool_knapsack, (&objects, 0), 10);
    objects.push((30, 0));
    test_case!(bool_knapsack, (&objects, 0), 40);
    objects.push((200, 0));
    test_case!(bool_knapsack, (&objects, 0), 240);
}

#[test]
fn test_take_all() {
    let objects = vec![(10, 20), (1, 20), (1, 1), (2, 2)];
    let (mut max_val, mut max_cap) = (0, 0);
    for (v, w) in objects.iter() {
        max_val += v;
        max_cap += w;
    }
    test_case!(bool_knapsack, (&objects, max_cap), max_val);
}

#[test]
fn test_general() {
    let objects = vec![(60, 10), (100, 20), (120, 30)];
    test_case!(bool_knapsack, (&objects, 50), 220);
    test_case!(bool_knapsack, (&objects, 10), 60);
    test_case!(bool_knapsack, (&objects, 30), 160);
}
