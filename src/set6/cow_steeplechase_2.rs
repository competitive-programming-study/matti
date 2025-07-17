// /
// / **COW STEEPLECHASE**
// / 
// / Given a set of n 2d segments (x_1:i64,y_1;i64,x_2:i64,y_2:i64) find the earliest segment that removed resort
// / in a set of not intersecting segments
// / 
// / **Strategy**
// / Since we have to return the first segment that intersects, we have to process each segment in the order that
// / they appear on the set. We initialize a lookup table for each segment, It will store the index of the segment
// / which intersects with. (Intersection is simmetric so if x intersects with y then y intersects with x).
// / 
// / We have to order segments coordinates in order to process from left to right, we tag them `Begin` and `End` and
// / we give them a segment index. We also use a intersection array as a container for each segment to mark intersection.
// / 
// / 
// #[derive(PartialEq,PartialOrd,Eq,Ord,Debug,Clone,Copy)]
// pub enum Segment{
//     Start,
//     End,
// }

// use std::collections::BTreeSet;

// fn counter_clockwise(a: (i64,i64), b: (i64,i64), c: (i64,i64)) -> bool {
//     (c.1 - a.1) * (b.0 - a.0) > (b.1 - a.1) * (c.0 - a.0)
// }

// fn intersect(i: (i64,i64,i64,i64), j: (i64,i64,i64,i64)) -> bool {
//     let a = (i.0,i.1);
//     let b = (i.2,i.3);
//     let c = (j.0,j.1);
//     let d = (j.2,j.3);
//     counter_clockwise(a,c,d) != counter_clockwise(b,c,d) && 
//     counter_clockwise(a,b,c) != counter_clockwise(a,b,d)
// }

// pub fn cow_steeplechase_2(segments: &[(i64,i64,i64,i64)]) -> usize {

//     //we store indexes for every intersection with the current index
//     let intersect_lookup: Vec<i32> = vec![-1;segments.len()]; 

//     let points = segments
//         .iter().
//         map(|&p|
//             if p.2 < p.1 {
//                 (p.2,p.3,p.0,p.1)
//             } else {p}
//         ).enumerate().flat_map(|(i,p)|
//             [(p.0,p.1,Segment::Start,i),(p.2,p.3,Segment::End,i)]
//         ).collect::<Vec<_>>();

//     points.sort_unstable();

//     let set: BTreeSet<(i64,i64,Segment,usize)> = BTreeSet::new();
    

//     for (_,_,event,i) in points {
//         if event == Segment::Start {
//             set.insert((x,y,event,index));
//         } else {    //Segment end for index i
//             //find all segments between Start(i) and End(i)
//             for &(_,_,_,j) in set.range((i64::MIN,i64::MIN,Segment::Start,index)..) {
//                 //find the corresponding segments and check for intersection
//                 if intersect(segments[i],segments[j]) {
//                     if intersect_lookup[j] != -1 {
//                         return j;
//                     } else {
//                         intersect_lookup[j] = i;
//                     }

//                     if intersect_lookup[i] != -1 {
//                         return i;
//                     } else {
//                         intersect_lookup[i] = j;
//                     }
//                 }
//             }
//         }
//     }


// }