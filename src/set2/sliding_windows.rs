#![allow(unused)]
use std::{
    collections::{BTreeSet, BinaryHeap, VecDeque},
    fmt::Binary,
};
/**
 * Brute-force
 */
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut maxs: Vec<i32> = Vec::new();
    let k = k as usize;
    for i in 0..nums.len() - (k - 1) {
        let mut max: i32 = nums[i];

        for &e in nums.iter().skip(i + 1).take(k - 1) {
            if e > max {
                max = e;
            }
        }
        maxs.push(max);
    }

    maxs
}

pub fn indeomatic_bruteforce(nums: Vec<i32>, k: i32) -> Vec<i32> {
    nums.windows(k as usize)
        .map(|w| *w.iter().max().unwrap())
        .collect()
}

/**
 * We use a TreeSet for memoization, this implies we have
 * insert:  log(n)
 * remove:  log(n)
 * last:    log(n)
 */
pub fn max_sliding_window_bst(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let (mut n, k) = (nums.len(), k as usize);

    if n < k {
        return Vec::new();
    };

    //alloc for max elements
    let mut maxs: Vec<i32> = Vec::with_capacity(n - k + 1);
    let mut set: BTreeSet<(i32, usize)> = BTreeSet::new();

    let mut max_so_far = *nums.first().unwrap();

    for (i, &v) in nums.iter().enumerate() {
        set.insert((v, i)); //insert the element

        max_so_far = max_so_far.max(v); //update the max if needed

        //if we're processing the second window
        if i >= k {
            //evict from set
            set.remove(&(nums[i - k], i - k));
            //if max_so_far value was equal to the dropped item
            if max_so_far == nums[i - k] {
                max_so_far = set.last().unwrap().0; //first element of the tuple
            }
        }
        //if we finished processing the first window
        if i >= (k - 1) {
            maxs.push(max_so_far);
        }
    }

    maxs
}

pub fn max_sliding_window_heap(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let (mut n, k) = (nums.len(), k as usize);

    if n < k {
        return Vec::new();
    };

    //alloc for max elements
    let mut maxs: Vec<i32> = Vec::with_capacity(n - k + 1);
    let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();

    //push the first window
    for (i, &v) in nums.iter().take(k).enumerate() {
        heap.push((v, i));
    }

    //starting processing the second window
    for (i, &v) in nums.iter().skip(k).enumerate() {
        heap.push((v, i)); //push the element in the window

        //remove from the heap all maxs that are not in the current window
        while let Some((_, idx)) = heap.peek() {
            if *idx != i - (k - 1) {
                heap.pop();
            } else {
                break;
            }
        }

        //insert the max item of the heap
        maxs.push(heap.peek().unwrap().0);
    }

    maxs
}

pub fn max_sliding_window_deque(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let (mut n, k) = (nums.len(), k as usize);

    if n < k {
        return Vec::new();
    };

    //alloc for max elements
    let mut maxs: Vec<i32> = Vec::with_capacity(n - k + 1);
    let mut d = VecDeque::<usize>::new();

    //process the first window
    //if
    for (i, &v) in nums.iter().take(k).enumerate() {
        if !d.is_empty() && v > nums[*d.back().unwrap()] {
            // new value is not max
            d.pop_back();
        } else {
            d.push_back(i)
        }
    }

    maxs.push(nums[*d.front().unwrap()]);

    for (i, &v) in nums.iter().skip(k).enumerate() {
        while let Some(&idx) = d.front() {
            if idx <= i - k {
                d.pop_front();
            }
        }

        while let Some(&idx) = d.back()
            && v > nums[idx]
        {
            d.pop_back();
        }
        d.push_back(i);
        maxs.push(nums[*d.front().unwrap()]);
    }
    maxs
}
