#![allow(unused)]

/// **MISSING INTEGER (Gauss)**
///
/// Given an array of n distinct unsigned integers in the range [0,n] (potentially unsorted),
/// we have to return the only missing integer
///
/// Returns an `Option<u32>`
///
/// By problem specs, we know exactely one integer is missing so we can use the Gauss sum to compute
/// the sum of all integers. By subtracting all integers in the range we find the missing one
///
/// *Space Complexity:  O(1)*
///
/// *Time Complexity:   O(n)*
///
pub fn missing_integer_gauss(arr: &[u32]) -> Option<u32> {
    //Compute the sum of the n integers in the array
    let mut sum: u32 = ((arr.len() * (arr.len() + 1)) / 2) as u32;

    if sum == 0 {
        //array is empty
        None
    } else {
        Some(sum - arr.iter().sum::<u32>())
    }
}

/// **MISSING INTEGER (XOR)**
///
///Given an array of n distinct unsigned integers in the range \[0,n\] (potentially unsorted),
///we have to return the only missing integer
///
///Returns an `Option<u32>`
///
///`XOR` is a (bit-a-bit) commutative and associative operator, so that:
///* `x ^ 0 = 0`
///* `x ^ x = 0`
///
///By problem specs, we can deduce that every value in the array can be mapped to
///the cell of `idx.eq(val)`, but the inverse (map every index to the values) is not
///true, (since one item is missing)
///
///Since `XOR` is commutative and associative and `(x ^ x) = 0`, we can have
///* `v0 ^ idx(0) ^ v1 ^ idx(1) ... vn ^ idx(n)`
///
///All the `(v_i,idx(i))` will cancel out beside the `idx` that can't be mapped to a value
///which corresponds to the missing integer
///
///**IMPORTANT**: the missing integer could be exactely n, which doesn't map to any index
///(even it's present) since arrays are 0-based, so we add it to the sum
///
///*Space Complexity: O(1)*
///
///*Time Complexity: O(n)*   
///
pub fn missing_integer_xor(arr: &[u32]) -> Option<u32> {
    if arr.is_empty() {
        return None;
    };

    let ln = arr.len() as u32;

    //Compute XOR of all the numbers in [0,n]
    //O(1) complexity
    let xor_idx = match ln & 3 {
        0 => ln,
        1 => 1,
        2 => ln + 1,
        _ => 0,
    };

    let xor_val = arr.iter().fold(0, |acc, x| x ^ acc);

    Some(xor_val ^ xor_idx)
}

/// **MISSING INTEGER (Mark)**
///
/// Given an array of n distinct unsigned integers in the range \[0,n\] (potentially unsorted),
/// we have to return the only missing integer
///
/// Returns an `Option<u32>`
///
/// Based on the conclusion that each value can be mapped to a specific index, we can swap the
/// items to their ordered cells by marking each swap with -1 (since all values are u32)
///
/// The only cell that at the end is setted to -1 corresponds to the missing integer. If no cell is
/// setted the missing integer is the max value.
///
/// Swapping a value to its ordered cell, can trigger a loop since at the end of the swap we would
/// have to swap the value that was contained in the cell. This loop is bounded, and at the worst
/// case (all values are in the wrong cells) we perform the loop n times, but then the vector is ordered
///
/// **IMPORTANT**: do it with a temporary vector is trivial, but to keep the function signature unaltered
/// we create a temporary vector, on which we perform this in place
///
/// *Space Complexity: O(1)* (if array as moved or mutable reference)
///
/// *Time Complexity: 2*n ~ O(n)*
///
pub fn missing_integer_mark(a: &[u32]) -> Option<u32> {
    if a.is_empty() {
        return None;
    }

    let mut arr = a.iter().map(|&x| x as i32).collect::<Vec<i32>>();
    let ln = arr.len() as i32;

    for i in 0..(arr.len()) {
        //Don't swap if the value is in the right cell or no value (-1)
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
            std::mem::swap(&mut arr[j as usize], &mut j);
        }
    }

    for (i, &val) in arr.iter().enumerate() {
        if val == -1 {
            return Some(i as u32);
        }
    }

    Some(ln as u32)
}

/// **MISSING INTEGER (Swap)**
///
/// Given an array of n distinct unsigned integers in the range \[0,n\] (potentially unsorted),
/// we have to return the only missing integer
///
/// Returns an `Option<u32>`
///
/// Based on the conclusion that each value can be mapped to a specific index, we can swap the
/// items to their ordered cells.
///
/// **IMPORTANT**: do it with a temporary vector is trivial, but to keep the function signature unaltered
/// we create a temporary vector, on which we perform this in place
///
/// *Space Complexity: O(1)* (if array as moved or mutable reference)
///
/// *Time Complexity: 2*n ~ O(n)*
///
pub fn missing_integer_swap(a: &[u32]) -> Option<u32> {
    if a.is_empty() {
        return None;
    }
    let mut v = a.to_vec();

    let mut i = 0;
    let ln = v.len();

    while i < ln {
        let val = v[i] as usize;

        //check if within range and if out of order
        if val < ln && v[i] != i as u32 {
            v.swap(i, val);
        } else {
            i += 1;
        }
    }

    for (i, &e) in v.iter().enumerate() {
        if e as usize != i {
            return Some(i as u32);
        }
    }

    Some(ln as u32)
}
