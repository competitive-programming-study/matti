use std::cmp::Ordering::*;
/**
 * Given a sorted array, return the start and end index of a specified target
 */
pub fn binary_search(nums: &Vec<i32>, target: i32, left: bool) -> Option<usize> {
    if nums.is_empty() {
        return None;
    }

    if nums.len() == 1 {
        return match nums.first().unwrap().cmp(&target) {
            Equal => Some(0),
            _ => None,
        };
    }

    let mut res = None;
    let (mut start, mut end): (usize, usize) = (0, nums.len());

    while start <= end {
        let mid = (start + end) / 2;
        match nums[mid].cmp(&target) {
            Equal => {
                res = Some(mid);
                if !left {
                    start = mid + 1;
                } else if mid == 0 {
                    break;
                } else {
                    end = mid - 1;
                }
            }
            Greater => {
                if mid == 0 {
                    break;
                }
                end = mid - 1;
            }
            Less => {
                start = mid + 1;
            }
        };
    }

    res
}

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    match binary_search(&nums, target, true) {
        Some(v) => {
            vec![
                v as i32,
                binary_search(&nums, target, false).unwrap() as i32,
            ]
        }
        None => {
            vec![-1; 2]
        }
    }
}

pub fn find_min(nums: Vec<i32>) -> i32 {
    let (mut start, mut end) = (0usize, nums.len() - 1);

    while start <= end {
        let mid = (start + end) / 2;

        match nums[mid].cmp(&nums[end]) {
            Greater => start = mid + 1,
            Less => end = mid,
            _ => end -= 1,
        }
    }

    nums[start]
}

pub fn peak(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return i32::MIN;
    };
    let (mut start, mut end) = (0usize, nums.len() - 1);

    //breaks when array is leq than 2 elements
    while (end - start) >= 2 {
        let mid = (start + end) / 2;

        let (lv, mv, rv) = (nums[mid - 1], nums[mid], nums[mid + 1]);
        // println!("start {start}, end {end}");
        match (mv.cmp(&lv), mv.cmp(&rv)) {
            (Greater, Greater) => return mid as i32,
            (Greater, Less) => start = mid + 1,
            _ => end = mid - 1, //arbitrariamente si sposta a sinistra
        };
    }

    if nums[start] > nums[end] {
        start as i32
    } else {
        end as i32
    }
}
