#![allow(unused)]
use std::{
    collections::{BTreeSet, BinaryHeap, VecDeque}
};

/// **MAX SLIDING WINDOW (bruteforce)**
/// 
/// Given a vector of integers (`i32`) and an unsigned integer, return
/// for all windows of size k the max value
/// 
/// Returns an `Option<Vec<i32>>`
/// 
/// **Example**
/// * `a = [7,2,5,3,4,3]`
/// * `k = 3`
/// 
/// * `win(0) = [7,2,5]`, max = 7
/// * `win(1) = [2,5,3]`, max = 5
/// * `win(2) = [5,3,4]`, max = 3
/// * `win(3) = [3,4,3]`, max = 4
/// 
/// The bruteforce approach consists on iterating on the whole window for every
/// window (even if when a window changes, only the first and last change)
/// 
/// *Space Complexity*:     O(1)
/// 
/// *Time Complexity*:      O(n * k)
/// 
pub fn max_sliding_bruteforce(nums: Vec<i32>, k: usize) -> Option<Vec<i32>> {
    if nums.len() < k || k == 0 {
        return None;
    }
    let windows = (nums.len() - k) + 1;

    let mut maxs: Vec<i32> = Vec::with_capacity(windows);

    for i in 0..windows {
        let mut max: i32 = nums[i];

        for &e in nums.iter().skip(i + 1).take(k - 1) {
            if e > max {
                max = e;
            }
        }
        maxs.push(max);
    }

    Some(maxs)
}

/// **MAX SLIDING WINDOW (bruteforce)**
/// 
/// Given a vector of integers (`i32`) and an unsigned integer, return
/// for all windows of size k the max value
/// 
/// **Example**
/// * `a = [7,2,5,3,4,3]`
/// * `k = 3`
/// 
/// * `win(0) = [7,2,5]`, max = 7
/// * `win(1) = [2,5,3]`, max = 5
/// * `win(2) = [5,3,4]`, max = 3
/// * `win(3) = [3,4,3]`, max = 4
/// 
/// Still a bruteforce approach but 
/// 
/// *Space Complexity*:     O(1)
/// *Time Complexity*:      O(n * k)
/// 
pub fn max_sliding_ideomatic(nums: Vec<i32>, k: usize) -> Option<Vec<i32>> {
    if nums.len() < k || k == 0{
        return None;
    }

    Some(nums.windows(k as usize)
        .map(|w| *w.iter().max().unwrap())
        .collect()
    )
}

/// **MAX SLIDING WINDOW (memoization Tree Set)**
/// 
/// Given a vector of integers (`i32`) and an unsigned integer, return
/// for all windows of size k the max value
/// 
/// **Example**
/// * `a = [7,2,5,3,4,3]`
/// * `k = 3`
/// 
/// * `win(0) = [7,2,5]`, max = 7
/// * `win(1) = [2,5,3]`, max = 5
/// * `win(2) = [5,3,4]`, max = 3
/// * `win(3) = [3,4,3]`, max = 4
/// 
/// We use a Tree Set data structure which holds tuples of the form `(u32,usize)`.
/// A TreeSet guarantees logarithmic time complexity for operations like:
/// - insert_key
/// - extract_key
/// - max_key
/// - min_key
/// 
/// **IMPORTANT**
/// 
/// TreeSet guarantees logarithmic complexity by using an ordering policy 
/// to organize the elements. Since we use tuples, by default the values are indexed by
/// the first item of the tuple in ascending order.
/// 
/// *Process First Window*
/// 
/// We insert all items in the first window in the TreeSet, while also keeping
/// track of the max_so_far. (At the end we push the current max to the return vector).
/// 
/// *Process other windows*
/// 
/// While inserting items (from the second to the last window), we remove the first item
/// of the previous window (can be done since we know the index). If the item removed
/// was equal to the current max, we update it with the set max O(log(k))
/// 
/// *Space Complexity*:     O(k)
/// 
/// *Time Complexity*:      O(n * log(k))
/// 
pub fn max_sliding_window_bst(nums: Vec<i32>, k: usize) -> Option<Vec<i32>> {
    let ln = nums.len();
    if ln < k || k == 0 {
        return None;
    }
    let mut maxs: Vec<i32> = Vec::with_capacity((ln - k) + 1);
    let mut set: BTreeSet<(i32, usize)> = BTreeSet::new();
    let mut max_so_far = *nums.first().unwrap();
    let mut i = 0;

    //process the first window
    while i < k {
        set.insert((nums[i],i));
        max_so_far = nums[i].max(max_so_far);
        i+=1;
    }

    //process all other windows
    while i < ln {
        //push the max of the previous window
        maxs.push(max_so_far);

        let last_win = (nums[i-k],i-k);
        set.remove(&last_win);

        //update the max if needed
        if max_so_far == last_win.0 {
            max_so_far = set.last().unwrap().0;
        }
        i+=1;
    }

    Some(maxs)
}


/// **MAX SLIDING WINDOW (memoization Heap)**
/// 
/// Given a vector of integers (`i32`) and an unsigned integer, return
/// for all windows of size k the max value
/// 
/// **Example**
/// * `a = [7,2,5,3,4,3]`
/// * `k = 3`
/// 
/// * `win(0) = [7,2,5]`, max = 7
/// * `win(1) = [2,5,3]`, max = 5
/// * `win(2) = [5,3,4]`, max = 3
/// * `win(3) = [3,4,3]`, max = 4
/// 
/// We use a Max-Heap data structure that allows for O(1) complexity for retreiving the max value in
/// the heap. We still have logarithmic complexity for inserting and extracting.
/// 
/// **IMPORTANT**
/// 
/// Binary-Heap guarantees logarithmic complexity by using an ordering policy 
/// to organize the elements. Since we use tuples, by default the values are indexed by
/// the first item of the tuple in ascending order.
/// 
/// *Process First Window*
/// 
/// We insert all items in the first window in the Heap.
/// 
/// *Process other windows*
/// While inserting items (from the second to the last window), we peek into the heap, removing
/// all max values that are not in our current window
/// 
/// *Space Complexity*:     O(n)
/// 
/// *Time Complexity*:      O(n * log(n)) 
/// 
/// Complexity is worse than the TreeSet case since we could accumulate items 
/// that we don't need (other windows) in the heap
/// 
/// 
pub fn max_sliding_window_heap(nums: Vec<i32>, k: usize) -> Option<Vec<i32>> {
    let ln = nums.len();
    if ln < k || k == 0 {
        return None;
    }

    let mut maxs: Vec<i32> = Vec::with_capacity((ln - k) + 1);
    let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();
    let mut i = 0;

    //Process the first window
    while i < k {
        heap.push((nums[i],i));
        i+=1;
    }

    //Process all other windows
    while i < ln {
        maxs.push(heap.peek().unwrap().0);
        heap.push((nums[i],i));

        while let Some((_,idx)) = heap.peek() {
            if *idx != i - (k - 1) {    //Evict max elements that are not in the current window
                heap.pop();
            } else {
                break;
            }
        }
        i+=1;
    }

    Some(maxs)
}

/// **MAX SLIDING WINDOW (memoization Tree Set)**
/// 
/// Given a vector of integers (`i32`) and an unsigned integer, return
/// for all windows of size k the max value
/// 
/// **Example**
/// * `a = [7,2,5,3,4,3]`
/// * `k = 3`
/// 
/// * `win(0) = [7,2,5]`, max = 7
/// * `win(1) = [2,5,3]`, max = 5
/// * `win(2) = [5,3,4]`, max = 3
/// * `win(3) = [3,4,3]`, max = 4
/// 
/// We use a Deque data structure that allows for O(1) complexity for insertion at the top and back of 
/// the queue
/// 
/// *Process First Window*
/// 
/// We start inserting items in the deque
/// 
/// *Process other windows*
/// While inserting items (from the second to the last window), we peek into the heap, removing
/// all max values that are not in our current window
/// 
/// *Space Complexity*:     O(n)
/// 
/// *Time Complexity*:      O(n * log(n)) 
/// 
/// Complexity is worse than the TreeSet case since we could accumulate items 
/// that we don't need (other windows) in the heap
/// 
/// 
pub fn max_sliding_window_deque(nums: Vec<i32>, k: usize) -> Option<Vec<i32>> {
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
