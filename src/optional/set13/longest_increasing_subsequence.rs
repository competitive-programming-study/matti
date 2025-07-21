///
/// ## Longest increasing subsequence
///
/// Given an array of integers return the length of the
/// longest increasing subsequence. An increasing subsequence
/// is defined as a sequence of elements where s[i + j] > s[i] for
/// positive i,j.
///
/// ## Returns
/// an usize that represents the longest length
///
/// ## Strategy
/// We use a dynamic programming array with a number of cells that is equal
/// to the list length. All cells are initialized with 1. Each cell will contain
/// the length of the longest increasing subsequent of the list truncated at the
/// ith index
///
/// We iterate on this array, and for each cell i, we iterate on the first i elements
/// updating the value of the cell only if the item we're iterating is bigger than the
/// ith item.
///
/// If so we update the cell with the max between its current value and the value of the
/// cell j + 1.
///
/// ## Complexity
/// With this approach we have O(n^2) time complexity and O(n) space complexity.
///
pub fn longest_increasing_subsequence(list: &[i32]) -> usize {
    if list.is_empty() {
        return 0;
    }
    let mut mem = vec![1; list.len()];
    for i in 1..mem.len() {
        for j in 0..i {
            if list[j] < list[i] {
                //if found a smaller element
                mem[i] = mem[i].max(mem[j] + 1);
            }
        }
    }
    *mem.iter().max().unwrap()
}

///
/// ## Longest increasing subsequence
///
/// Given an array of integers return the length of the
/// longest increasing subsequence. An increasing subsequence
/// is defined as a sequence of elements where s[i + j] > s[i] for
/// positive i,j.
///
/// ## Returns
/// an usize that represents the longest length
///
/// ## Strategy
/// We use an additional vector which we use to store the smallest
/// possible tail value for the list at every length.
///
/// Iterate on the numbers, and look for each number in the vector,
/// the vector is sorted so we can do it with a binary search. The
/// binary search returns the index of the first item greater or equal
/// than the one provided. If the index is higher than the length of
/// the vector then we append the item, else we replace it.
///
/// At the end the vector will store the longest subsequence
///
/// ## Complexity
/// To implement this we require O(n) additional space and O(nlog(n)) time
/// since we perform binary search for all elements
///
pub fn longest_increasing_subsequence_binary(list: &[i32]) -> usize {
    if list.is_empty() {
        return 0;
    }
    let mut sub = Vec::with_capacity(list.len());
    for e in list {
        match sub.binary_search(e) {
            Ok(_) => {}
            Err(idx) => {
                if idx >= sub.len() {
                    sub.push(*e);
                } else {
                    sub[idx] = *e;
                }
            }
        }
    }
    sub.len()
}
