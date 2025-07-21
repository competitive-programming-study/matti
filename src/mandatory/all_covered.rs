/// # All integers in range covered
///
/// Given an array of intervals represented as tuples (begin: i32,end i32) and another interval
///  given by 2 parameters `left` and `right`.
///
/// ## Returns
/// `true` if the range is covered by the overlapping intervals
///
/// ## Strategy
/// We can't assume the intervals in the array are ordered in any way nor that consecutive intervals perfectly
/// match.
///
/// We build a coverage array, we need an integer for every item in the range [start,end]. Since each cell maps if
/// the corrisponding integer normalized (cell 0 maps left, cell 1 maps left + 1 and so on), is covered by any
/// provided interval.
///
/// Then we scan the intervals provided: we mark the corresponding cells.
///
/// Then we scan the coverage array marking each cell as the value of the previous one, in this way we can mark cells
/// that are covered by at least one interval
///
/// We scan the coverage array one last time, if at least one cell hasn't been marked then the intervals dont cover
/// the provided one
///
/// ## Complexity
/// We require O(n) (n being the amplitude of the range) time and space
pub fn all_covered(intervals: &[(i32, i32)], int: (i32, i32)) -> bool {
    let (left, right) = (int.0, int.1);
    let size = (right - left + 2) as usize; // +2 to safely decrement beyond right
    let mut coverage = vec![0; size];

    for (s, e) in intervals {
        let start = *s.max(&left);
        let end = *e.min(&right);
        if start > end {
            continue; // No overlap with [left, right]
        }
        coverage[(start - left) as usize] += 1;
        if ((end + 1 - left) as usize) < size {
            coverage[(end + 1 - left) as usize] -= 1;
        }
    }

    // build coverage count at each point
    for i in 1..(size - 1) {
        coverage[i] += coverage[i - 1];
    }

    // check if every point is covered
    coverage[..(size - 1)].iter().all(|&c| c > 0)
}

#[derive(PartialOrd, PartialEq, Ord, Eq, Debug, Clone, Copy)]
pub enum Event {
    Begin,
    End,
}

/// # All integers covered (sweep algorithm)
///
/// Implementation of the covering_intervals as sweep algorithm.
///
/// ## Strategy
/// 1. Sort and flatten the intervals, marking each start as an `Event::Begin` and each end as `Event::End`
/// 2. early return false if the first point begin is higher than our start, same thing for end
/// 3. check if any sub-interval of int is covered by at least one interval
///
/// ## Returns
/// `true` if the provided interval is comprised in the other ones
///
///
/// ## Complexity
/// The time complexity is O(nlog(n)) due to initial sorting, we require linear
/// time to process. We require O(m) additional space to store the sorted segments
pub fn all_covered_sweep(intervals: &[(i32, i32)], int: (i32, i32)) -> bool {
    if intervals.is_empty() {
        return false;
    }
    let (start, end) = (int.0, int.1);
    let mut pairs: Vec<(i32, Event)> = intervals
        .iter()
        .flat_map(|&(s, e)| ([(s, Event::Begin), (e, Event::End)]))
        .collect();

    pairs.sort_unstable();
    let mut coverage = 0;

    //used to check (p,Event::End),(p+1,Event::Begin)
    let mut last_active: i32 = 0;

    //Early check: if coverage starts after `start` or ends before `end`
    if pairs[0].0 > start || pairs.last().unwrap().0 < end {
        return false;
    }

    for (p, event) in pairs {
        //don't need to process points after `end`
        if p >= end {
            break;
        }
        if event == Event::Begin {
            //account for intervals that can end and begin back at next integer
            if coverage == 0 && p > start && last_active != (p - 1) {
                return false;
            }
            coverage += 1;
        } else {
            coverage -= 1;
            //set p as last active [will be checked again if the next point is a begin]
            //else coverage is broken
            if coverage == 0 {
                last_active = p;
            }
            //return
            if coverage < 0 {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod test_all_covered {
    use super::*;

    #[test]
    fn test_1() {
        let v = [(1, 2), (3, 4), (5, 6)];
        let int = (2, 5);
        assert_eq!(all_covered(&v, int), true);
        assert_eq!(all_covered_sweep(&v, int), true);
    }

    #[test]
    fn test_2() {
        let v = [(1, 10), (10, 20)];
        let int = (21, 21);
        assert_eq!(all_covered(&v, int), false);
        assert_eq!(all_covered_sweep(&v, int), false);
    }

    #[test]
    fn test_3() {
        let v = [
            (8, 31),
            (2, 20),
            (22, 25),
            (13, 27),
            (7, 17),
            (17, 47),
            (14, 33),
            (28, 39),
        ];
        let int = (2, 15);
        assert_eq!(all_covered(&v, int), true);
        assert_eq!(all_covered_sweep(&v, int), true);
    }
}
