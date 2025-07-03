#![allow(unused)]

/**
 * Given an array of n unsigned integers in the range [0,n] we have to return
 * the only missing integer
 *
 * Runtime: O(n)
 *
 * We use the Gauss summation to compute the sum of the first n integers, then
 * we subtract it to the array items. The left of sum will be the integer we're looking
 * for
 */

pub fn missing_integer(arr: &[u32]) -> u32 {
    if arr.is_empty() {
        panic!("Not supported input");
    };

    //Calculate the sum of the n integers in the array
    let mut sum: u32 = ((arr.len() * (arr.len() + 1)) / 2) as u32;

    for e in arr.iter() {
        sum -= *e;
    }
    sum
}

/**
 * XOR is commutative and associative so that:
 * x XOR x = 0
 * x XOR 0 = x
 *
 * We scan the array taking the XOR of the indexes as well as the XOR of
 * the items:
 * We know for sure that each item can be mapped only one index, but there is
 * only one index that can't be mapped to an item.
 *
 * Since xor is commutative and associative well'have that items that map to
 * an index will cancel out with the index while the only item that doesn't
 * will be the result
 *
 * Runtime: O(n)
 */
pub fn missing_integer_xor(arr: &[u32]) -> u32 {
    if arr.is_empty() {
        panic!("Not supported input");
    };

    let (mut xor_idx, mut xor_items) = (0u32, 0u32);

    for (v, e) in arr.iter().enumerate() {
        xor_idx ^= v as u32;
        xor_items ^= *e;
    }

    //We also xor with the
    xor_idx ^ xor_items ^ (arr.len() as u32)
}

/**
 * Based on the conclusion that each item can be mapped to an index (unless it's)
 * the max item, we can place each item in the corresponding cell while setting
 * the cell to -1 if the item is out-of-order, in the end we can scan the whole
 * vector, and the only cell that is -1 corresponds to the item missing.
 *
 * If all cells are filled then the item missing is the max
 *
 * This can be run in-place on a mutable vector-or-array type or by creating a copy
 *
 * Do it with a copy is is easy so we just keep the function interface immutable and
 * we do it in-place on a local copy
 *
 *
 */
pub fn missing_integer_mark(a: &[u32]) -> u32 {
    let mut arr: Vec<i32> = a.iter().map(|&x| x as i32).collect();
    let ln = arr.len() as i32;

    for i in 0..arr.len() {
        if arr[i] == i as i32 || arr[i] == -1 {
            continue;
        }

        //new index to walk
        let mut j = arr[i];
        //set the cell to -1
        arr[i] = -1;

        while j >= 0 && j < ln {
            //if cell is ordered then break
            if arr[j as usize] == j {
                break;
            }

            //next cell to walk
            let next = arr[j as usize];
            //order the cell
            arr[j as usize] = j;

            //update the index
            j = next;
        }
    }

    for (i, &val) in arr.iter().enumerate() {
        if val == -1 {
            return i as u32;
        }
    }

    ln as u32
}

/**
 * Same thing as above we can do it without marking it, just swap the elements in
 * the correct cells,
 *
 * We use an i as the index of the loop, and we increment it when the item == cell_idx
 * if not we swap i and the cell_item and we keep going.
 *
 * At the end of it, the first item that doesn't correspond to the index is the
 * one that's missing
 */
pub fn missing_integer_swap(a: &[u32]) -> u32 {
    let mut v = a.to_vec();
    let mut i = 0;
    while i < v.len() {
        let val = v[i] as usize;

        //check if within range and if out of order
        if val < v.len() && v[i] != i as u32 {
            v.swap(i, val);
        } else {
            i += 1;
        }
    }

    for (i, e) in v.iter().enumerate() {
        if *e as usize != i {
            return i as u32;
        }
    }

    v.len() as u32
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    static TO_TEST: [(&str, fn(&[u32]) -> u32); 4] = [
        ("GAUSS", missing_integer),
        ("MARK", missing_integer_mark),
        ("SWAP", missing_integer_swap),
        ("XOR", missing_integer_xor),
    ];

    struct TestCase {
        input: Vec<u32>,
        output: u32,
    }

    impl TestCase {
        fn check_input(v: &[u32]) -> i32 {
            let n = v.len() as u32;
            let expected_sum = (n * (n + 1)) / 2;

            let mut actual_sum = 0u32;
            let mut seen = HashSet::new();

            for &e in v {
                // Range check
                if e > n || !seen.insert(e) {
                    return -1; // Out of range or duplicate
                }
                actual_sum += e;
            }

            let missing = expected_sum as i32 - actual_sum as i32;

            // Check that missing is in valid range
            if missing < 0 || missing > n as i32 {
                return -1;
            }

            missing
        }

        fn new(i: Vec<u32>) -> Self {
            let out = TestCase::check_input(&i);
            if out == -1 {
                panic!("TestCase construction failed");
            }
            TestCase {
                input: (i),
                output: (out as u32),
            }
        }
    }

    #[test]
    fn test_all_implementations() {
        let test_cases = [
            TestCase::new(vec![3, 0, 1]),
            TestCase::new(vec![0, 1]),
            TestCase::new(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]),
        ];

        for case in test_cases {
            for (name, func) in TO_TEST.iter() {
                assert_eq!(
                    func(&case.input),
                    case.output,
                    "Failed on input {:?} with function {:?}",
                    case.input,
                    name
                );
            }
        }
    }
}
