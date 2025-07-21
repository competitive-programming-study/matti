///
/// # N meetings in a room
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

#[cfg(test)]
mod test_n_meetings {
    use super::*;

    #[test]
    fn test_empty() {
        let meetings = vec![];
        assert_eq!(n_meetings(&meetings), 0);
    }

    #[test]
    fn test_single_meeting() {
        let meetings = vec![(1, 2)];
        assert_eq!(n_meetings(&meetings), 1);
    }

    #[test]
    fn test_non_overlapping_meetings() {
        let meetings = vec![(1, 2), (3, 4), (5, 6)];
        assert_eq!(n_meetings(&meetings), 3);
    }

    #[test]
    fn test_completely_overlapping_meetings() {
        let meetings = vec![(1, 5), (2, 6), (3, 7)];
        assert_eq!(n_meetings(&meetings), 1);
    }

    #[test]
    fn test_partial_overlaps() {
        let meetings = vec![(1, 3), (2, 4), (3, 5), (6, 8)];
        // (1,3), (6,8) [ cant select (3,5) in the middle because first meeting ends at 3]
        assert_eq!(n_meetings(&meetings), 2);
    }

    #[test]
    fn test_edge_case_equal_end_and_start() {
        let meetings = vec![(1, 3), (3, 5), (5, 6)];
        //select (1,3),(5,6)
        assert_eq!(n_meetings(&meetings), 2);
    }

    #[test]
    fn test_multiple_valid_schedules() {
        let meetings = vec![(1, 4), (2, 3), (3, 5), (7, 9), (5, 8)];
        assert_eq!(n_meetings(&meetings), 2); // (2,3), (5,8) or (1,4), (5,8)
    }

    #[test]
    fn test_unsorted_input() {
        let meetings = vec![(8, 9), (1, 2), (3, 4), (0, 6), (5, 7)];
        assert_eq!(n_meetings(&meetings), 4); // (1,2), (3,4), (5,7), (8,9)
    }

    #[test]
    fn test_all_start_before_first_end() {
        let meetings = vec![(1, 10), (2, 3), (3, 4), (4, 5)];
        //select (2,3) and (4,5)
        assert_eq!(n_meetings(&meetings), 2);
    }
}
