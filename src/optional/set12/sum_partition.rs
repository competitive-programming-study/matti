///
/// ## Partition Sum
/// Given an unsigned integer multiset, we want to know if we
/// can partition it in 2 subsets so that they have the same
/// size
///
/// ## Returns
/// True or false according to the partition scheme
///
/// ## Strategy
/// We will adopt a dynamic programming approach since checking for
/// every possible subset is exponential in the set size.
///
/// We first can say that if the sum is not even, then we can't partition
/// in 2 subsets.
///
/// We use a memoization boolean array has the same size as the target sum
/// we're trying to reach. The target sum is always defined as the total sum
/// + 1 (since we account for 0 sum).
///
/// We initialzie all cells with false, exception for the the first cell that
/// marks sum = 0, since the empty set has sum 0.
///
/// For the i'th cell we iterate through the slice marking the cell(i) as true if
/// the cell i-v was true. Logically marking a cell as true, means we found a partition
/// that makes up that cell
///
/// ## Complexity
/// The time complexity is linear with respect to the size of the initial set as
/// well as the space complexity
///
pub fn sum_partition(slice: &[usize]) -> bool {
    let sum_items = slice.iter().sum::<usize>();
    if sum_items % 2 != 0 {
        return false;
    }
    let target_sum: usize = sum_items / 2;

    let mut mem = vec![false; target_sum + 1];
    mem[0] = true; //mark the cell 0 as true since sum 0 is always achieved by the empty set

    for &v in slice {
        for j in (v..=target_sum).rev() {
            if mem[j - v] {
                mem[j] = true;
            }
        }
    }

    *mem.last().unwrap()
}
