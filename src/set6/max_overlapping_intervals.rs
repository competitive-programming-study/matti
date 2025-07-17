#[derive(PartialOrd,PartialEq,Ord,Eq,Debug)]
enum Event {
    Begin,
    End
}

/// **MAX OVERLAPPING INTERVALS**
/// 
/// Given an array of intervals represented as tuples `(begin: usize, end: usize)` return the maximum number of overlapping intervals at
/// a point x.
/// 
/// **Strategy**
/// The idea here, is to adopt the sweep line algorithm, so having an imaginary line that sweeps every point (we mean interval point so begin/end) and
/// keeps track of the best result.
/// 
/// In order to do this the array of interval needs some processing: in the following examples we're using an enum `Event`, that we use to tag every
/// interval producing 2 values for each. Both values are tuples and the interval begin is tagged as an `Event::Begin`, simmetric for interval end.
/// 
/// The intervals are flattened out, and sorted (the `Event enum` derives the necessary traits in order to be a sort key as well). 
/// 
/// At every point then we check the tag item, incrementing or decrementing a counter to reflect the number of overlapping intervals at that point.
/// 
/// Then we return the max overlapping.
/// 
/// *Time Complexity*: O(nlog(n)) + O(n) ~ O(n(logn))
/// 
/// *Space Complexity*: O(n) in order to build the flat_map
/// 
pub fn max_overlapping_ideomatic(intervals: &[(usize,usize)]) -> usize {
    let mut pairs: Vec<(usize,Event)> = intervals
        .iter()
        .flat_map(|&(b,e)|[(b,Event::Begin),(e,Event::End)])
        .collect();

    pairs.sort_unstable();

    pairs
        .into_iter()
        .scan(0,|counter,(_,kind)| {
            if kind == Event::Begin {
                *counter += 1;
            } else {
                *counter -= 1;
            }
            Some(*counter)
        }).max().unwrap_or(0)

}


/// **MAX OVERLAPPING INTERVALS**
/// 
/// Same as `max_overlapping_ideomatic` but explicit on processing details
pub fn max_overlapping_explicit(intervals: &[(usize,usize)]) -> usize {
    let mut pairs: Vec<(usize,Event)> = Vec::with_capacity(intervals.len() * 2);

    for &(b,e) in intervals {
        pairs.push((b,Event::Begin));
        pairs.push((e,Event::End))
    };

    pairs.sort_unstable();
    let mut global_counter = 0;
    let mut current_counter = 0;

    for (_,kind) in pairs {
        if kind == Event::Begin {
            current_counter += 1;
        } else {
            current_counter -= 1;
        }
        global_counter = global_counter.max(current_counter);
    };

    global_counter
    
}