//euclidean distance squared
fn distance_squared(p:(i64,i64),q:(i64,i64)) -> i64 {
    (p.0 - q.0).pow(2) + (p.1 - q.1).pow(2)
}

///**CLOSEST PAIR OF POINTS (Bruteforce)**
/// 
/// given a set of n points on the plane (they can be m-dimensional but we'll consider them in N^2) we have
/// to compute the minimum distance of 2 points in the set
/// 
/// We inspect all possible pair of points.
/// 
/// *Time Complexity*: O(n^2)
/// 
pub fn closest_pair_bruteforce(points: &[(i64,i64)]) -> Option<i64> {
    let mut delta = i64::MAX;

    for i in 0..points.len() {
        for j in 0..points.len() {
            if i == j {
                continue;
            }
            delta = delta.min(distance_squared(points[i],points[j]));
        }
    }
    Some(delta)
}

///
/// **CLOSEST PAIR OF POINTS**
/// 
/// given a set of n points on the plane (they can be m-dimensional but we'll consider them in N^2) we have
/// to compute the minimum distance of 2 points in the set
/// 
/// We can employ the sweeping line paradigm by using the definition of euclidean distance using the sweeping
/// line to try to improve a suboptimal distance `delta` that we can initialize as the distance between the 
/// first 2 points.
/// 
/// Given a distance delta, and a point p of coordinates (x,y), we can improve the distance if the closest point
/// to the left of p, has a distance smaller than delta. If it exists it must have x-coordinate in the interval
/// [x-delta,x] and y coordinate in the interval [y-delta,y+delta];
/// 
/// The algorithm mantains a BST with points sorted by ther y-coordinates. When processing a point we iterate over
/// the points in the interval [y pm delta]. If the current point has a x-coordinate smaller than x-delta we remove
/// it from the set.
/// 
use std::{collections::BTreeSet,ops::Bound::Included};

pub fn closest_pair(points: &[(i64,i64)]) -> Option<i64> {
    if points.len() < 2 {
        return None;
    }
    let mut points = points.to_vec();
    //sort points by y coordinate
    points.sort_unstable_by_key(|p|(p.1,p.0));
    
    let (min_y,max_y) = (points[0].1,points.last()?.1);
    let mut delta = distance_squared(points[0], points[1]);

    let mut set: BTreeSet<(i64,i64)> = BTreeSet::new();

    for &(p_x,p_y) in points.iter() {
        
        //remove points with too small y-coordinate
        //
        //filter the points when y > coordinate is bigger than y - delta
        let delete: Vec<(i64,i64)> = set.range((
            Included(&(p_x - delta, min_y)),
            Included(&(p_x + delta, max_y))
        )).filter(|&p| p.1 > p.1 - delta).cloned().collect();

        //remove the points after the range iterator has been destroyed
        for p in delete {
            set.remove(&p);
        }

        //update delta selecting the min distance squared of the points left in the set
        //
        //the points in the the set have to have matching x coordinates (inside the delta-rectangle)
        delta = set
            .range((
                Included(&(p_x - delta,min_y)),
                Included(&(p_x + delta,max_y)),
            )).fold(delta,|acc,&p| acc.min(distance_squared(p, (p_x,p_y))));

        set.insert((p_x,p_y));
    }

    Some(delta)
}