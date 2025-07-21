use std::collections::BTreeSet;

/// # Frogs and Mosquitoes
///
/// We're given two arrays:
/// - `frogs`: Each frog is represented by `(x, tongue)`, where `x` is its position on the x-axis,
///   and `tongue` is the initial reach to the right (i.e., it can eat any mosquito in `(x, x + tongue]`).
/// - `mosquitos`: Each mosquito is represented by `(size, x)`, where `x` is its position, and `size`
///   is how much it extends a frog's tongue if eaten.
///
/// ## Constraints
/// 1. Mosquitoes arrive one at a time in the given order.
/// 2. A frog can eat a mosquito if the mosquito’s position is strictly within the frog’s reach `(x, x + tongue]`.
/// 3. If multiple frogs can reach a mosquito, the leftmost (smallest `x`) frog eats it.
/// 4. When a frog eats a mosquito, its tongue increases by the mosquito's size.
/// 5. A frog continues eating mosquitoes (including earlier uneaten ones) as long as new ones fall within reach.
///
/// ## Returns
/// A vector of `(mosquitoes_eaten, final_tongue_length)` for each frog in input order.
///
/// ## Strategy
/// - Store all frogs in a `BTreeSet`, ordered by `(max_reach = x + tongue, x, id)`
///   so we can efficiently find the frog with minimum x that can reach a mosquito.
/// - Use a lookup table (`Vec`) to track each frog’s position, tongue length, and number of mosquitoes eaten.
/// - Maintain a second `BTreeSet` for *pending mosquitoes* that no frog could reach at arrival time.
///
/// ### Processing
/// 1. Initialize the frog set with each frog’s reach and position.
/// 2. For each mosquito:
///     - Use a range query to find the first frog whose reach covers the mosquito's position.
///     - If found:
///         - Remove the frog from the set.
///         - Update its tongue and count.
///         - Check if it can now eat any pending mosquitoes, and process those.
///         - Re-insert the updated frog into the set.
///     - Otherwise, insert the mosquito into the pending set.
///
/// ### Frogs Ordering
/// Frogs are sorted by:
/// 1. `max_reach = x + tongue`
/// 2. `x` (position)
/// 3. `id` (index)
///
/// ## Complexity
/// We take O(nlog(n)) time to insert all frogs insie the BTree Set.
///
/// Foreach mosquitoe: Theta(m)
/// 1. look for the frog O(log(n))
/// 2. we can't eat the mosquito: have to store it O(log(m))
/// 3. we can eat the mosquito:
/// 4. remove the frog from the set O(log(n))
/// 5. at most, foreach mosquito pending: O(m)
/// 6. check if the frog can eat it O(log(n)) either remove it or not
///
/// The solution runs in O(nlog(n)) + O(m*log(n+m)) time.
///
/// We need 2 BTreeSet to store the frogs, and all the mosquitoe in the case no frogs
/// can eat them. This accounts for O(n*m);
pub fn frog_mosquitoes(frogs: &[(i32, i32)], mosquitos: &[(i32, i32)]) -> Vec<(usize, i32)> {
    type FrogLookup = (i32, i32, usize); // (max_reach, start, id)
    type FrogState = (i32, i32, usize); // (start, tongue, mosquitoes_eaten)
    type Mosquito = (i32, i32); // (position,size)

    //index each frog in the lookup table
    let mut lookup = frogs
        .iter()
        .map(|&(pos, tongue)| (pos, tongue, 0usize))
        .collect::<Vec<FrogState>>();

    //Create the TreeMap indexing frog by max_reach
    let mut frog_set: BTreeSet<FrogLookup> = BTreeSet::new();
    let mut pending_mosquitos: BTreeSet<Mosquito> = BTreeSet::new();

    // TreeSet Initialization: O(log(1) + log(2) + ... log(n)) = O(log(n!)) -> using Stirling Approx -> O(n log(n))
    for (i, &(pos, tongue, _)) in lookup.iter().enumerate() {
        let frog: FrogLookup = (pos + tongue, pos, i);
        frog_set.insert(frog);
    }

    for &(pos_m, size) in mosquitos {
        // Search frog that can eat
        // We want the frog such that (pos + tongue) >= mosquito.pos
        // Range returns an iterator, so we check if we have at least one frog
        if let Some(&(reach, pos_f, id)) = frog_set.range((pos_m, i32::MIN, usize::MIN)..).next() {
            if pos_f <= pos_m {
                //println!("Frog {id} can eat mosquito at position {pos_m}");
                //remove old entry
                frog_set.remove(&(reach, pos_f, id));

                //get frog fields from the lookup table
                let (start, tongue, eaten) = &mut lookup[id];

                //update frog attributes based on this mosquito
                *eaten += 1;
                *tongue += size;
                let mut new_reach = *start + *tongue + size;
                //println!("Frog {id} reach upgraded from {reach} to {new_reach}");

                //update frog state and keep checking if the new frog can eat pending mosquitos
                loop {
                    //we look for all mosquitos which position is lower or equal than our new reach

                    // We could iterate over all mosquitos in the range but we'd have to remove them
                    // and borrow-checker woudn't allow to perform mutable operations when an immutable
                    // iterator exists so the complexity is the same
                    if let Some(&(pos, size)) =
                        pending_mosquitos.range(..=(new_reach, i32::MAX)).next()
                    {
                        //frog can eat this so evict from set
                        if pos_f <= pos {
                            //println!("[Pending] Frog {id} can eat mosquito at position {pos}");

                            //evict the mosquito from the set
                            pending_mosquitos.remove(&(pos, size));

                            //update the frog state
                            *tongue += size;
                            *eaten += 1;
                            let _reach = new_reach; //for debug purposes
                            new_reach = *start + *tongue;

                            //println!("[Pending] Frog {id} reach upgraded from {_reach} to {new_reach}");
                            continue;
                        }
                    }

                    break;
                }

                //store the frog back
                frog_set.insert((new_reach, *start, id));
                continue; // the frog ate at least a mosquito so we don't insert it in pending
            }
        }

        //println!("Inserting Mosquito as pending ({pos_m},{size})");
        pending_mosquitos.insert((pos_m, size));
    }

    lookup
        .iter()
        .map(|&(_, tongue, eaten)| (eaten, tongue))
        .collect::<Vec<(usize, i32)>>()
}

#[cfg(test)]
mod test_frog_mosquitoes {
    use super::*;
    //Testcases from [[https://codeforces.com/contest/609/problem/F?locale=en]]

    #[test]
    fn test_1() {
        let frogs: Vec<(i32, i32)> = vec![(10, 2), (15, 0), (6, 1), (0, 1)];
        let mosquitos: Vec<(i32, i32)> =
            vec![(110, 10), (1, 1), (6, 0), (15, 10), (14, 100), (12, 2)];
        let expected: Vec<(usize, i32)> = vec![(3, 114), (1, 10), (1, 1), (1, 2)];

        assert_eq!(frog_mosquitoes(&frogs, &mosquitos), expected);
    }

    #[test]
    fn test_2() {
        let frogs: Vec<(i32, i32)> = vec![(10, 2)];
        let mosquitos: Vec<(i32, i32)> = vec![(20, 2), (12, 1)];
        let expected: Vec<(usize, i32)> = vec![(1, 3)];

        assert_eq!(frog_mosquitoes(&frogs, &mosquitos), expected);
    }
}
