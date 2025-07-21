///
/// ## N meetings in a room
///
/// Given an array of tuples each one representing a meeting with (start:usize,end:usize)
/// return the maximum numbers of meetings that can be executed without overlap.
///
/// Start time of chosen meeting cant be equal to end time of the previous meeting
///
/// ## Returns
/// an usize representing the max meetings
///
/// ## Strategy
/// With a greedy approach we sort meetings by smallest end time. We always track the
/// current endtime and select the next start that is greater than the current end time
///
/// ## Complexity
/// Time is O(nlog(n)) for ordering and space is O(1) if the provided array is mutable else
/// O(n)
pub fn n_meetings(meetings: &[(usize, usize)]) -> usize {
    if meetings.is_empty() {
        return 0;
    }
    let mut meeting_vec = meetings.to_vec();
    meeting_vec.sort_by_key(|&m| m.1);

    let mut curr_end = meeting_vec.first().unwrap().1;
    let mut count = 1;

    for &(start, end) in meeting_vec.iter().skip(1) {
        if start > curr_end {
            curr_end = end;
            count += 1;
        }
    }

    count
}
