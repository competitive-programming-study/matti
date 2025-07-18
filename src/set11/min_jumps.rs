/// ## Min jumps in array
///
/// Given an array of non negative numbers we start from index
/// 0 and we can jump up from 1 a[0] cells of the array. We have to
/// find the minimum numer of jumps to reach the last index
///
/// ## Returns
///
/// An `Option<usize>` which is `None` if the end of the array cant be
/// reached
///
/// ## Strategy
///
/// We use a memoization array where each cell is initialized to a MAX
/// value to mark them as undiscovered.
///
/// For each cell, we iterate through the value updating the corresponding
/// memoization cells with the minimum value between the current value and
/// the cell from which we're jumping from + 1 to record the jump.
///
/// ## Complexity
/// We use a memoization array which is O(n) in space. Iterating on the array
/// values, and each cell results in O(n^2) time complexity.
///
pub fn min_jumps(slice: &[usize]) -> Option<usize> {
    if slice.is_empty() {
        return None;
    } else if slice.len() == 1 && slice[0] != 0  {
        return Some(1);
    }
    let mut mem = vec![usize::MAX; slice.len()];
    mem[0] = 0;
    for i in 0..slice.len() {
        for j in 1..=slice[i] {
            if (i + j) >= mem.len() {
                break;
            }
            mem[i+j] = mem[i+j].min(mem[i] + 1);
        }
    }
    let res = *mem.last().unwrap();
    if res != usize::MAX { Some(res) } else { None }
}

/// ## Min jumps in array (GREEDY)
///
/// Given an array of non negative numbers we start from index
/// 0 and we can jump up from 1 a[0] cells of the array. We have to
/// find the minimum numer of jumps to reach the last index
///
/// ## Returns
///
/// An `Option<usize>` which is `None` if the end of the array cant be
/// reached
///
///
/// ## Strategy
/// Greedy algorithms use euristics in order to update partial solutions.
///
/// In this case we iterate on the array, keeping some counters:
/// - i: index of the cell we're currently on
/// - maxReach: index of the farthest cell we can reach
/// - jumps: jump counter
/// - currReach: coupled with i in order to update `jumps`
///
/// For each cell we traverse, we recompute maxReach as the max(maxReach, i + a[i])
///
/// - If maxReach is greater than the last index then we can return;
/// - Else if the currentReach is equal to the i index, we update it
/// to the current MaxReach incrementing the jump counter
/// - if the maxReach is equal to the i-th index then we can't jump further
/// so we return a failure
///
/// ## Complexity
/// We traverse the array one time and we use constant extra
/// space so it's O(n) in time and O(1) in space
///
pub fn min_jumps_greedy(slice: &[usize]) -> Option<usize> {
    let (mut jumps, mut max_reach, mut curr_reach) = (0, 0, 0);

    for i in 0..slice.len() {
        //update maxReach with the the max reachable cell
        max_reach = max_reach.max(i + slice[i]);
        //if we're out of the array then we return successfully

        //if the cell idx is the same as the current reach
        //we try to update it
        if curr_reach == i {
            curr_reach = max_reach;
            //if after the update it's the same we can't move further
            if curr_reach == i {
                return None;
            }
            jumps += 1;

            if curr_reach >= slice.len() - 1 {
                return Some(jumps);
            }
        }
    };
    None
}
