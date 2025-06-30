#![allow(unused)]

/**
 * We keep a local counter in which we accumulate the water if we're unsure there
 * exists a barrier high enough.
 * 
 * If we find the barrier, we update the barrier and commit the local water to the 
 * global water.
 * 
 * To handle this we need 2 sweeps, one from left to right the other in reverse, because
 * after the last (left) barrier there could be lower barriers that can't be caught
 * 
 * To do this we iterate in reverse until and comprising the last barrier index
 * 
 * Runtime: O(n)
 * 
 * At most 2 lookups per cell
 * 
 */
fn trap_2_pass(heights: &[i32]) -> i32 {


    let mut last_barrier_index = 0;
    let (mut barrier, mut local_water, mut water) = (0,0,0);

    for (i,&e) in heights.iter().enumerate() {
        if e >= barrier {
            barrier = e;
            last_barrier_index = i;
            water += local_water;
            local_water = 0;
        } else {
            local_water += barrier - e;
        }
    };

    (barrier,local_water) = (0,0);

    // do it in reverse until the barrier
    for &e in heights.iter().skip(last_barrier_index).rev() {
        
        if e >= barrier {
            water += local_water;
            barrier = e;
            local_water = 0;
        } else {
            local_water += barrier - e;
        }
        
    };

    return water;


}

/**
 * We use a two pointer (left, right) based approach, keeping a left max barrier 
 * and right max barrier.
 * 
 * We process left or right, based on which current cell is lowest, so we're sure
 * that there must exist at least one barrier that can contain our water.
 * 
 * We update the max pointers if we find that the current cell is higher, else we 
 * commit the water
 * 
 * Runtime O(n)
 * 
 * Only one lookup per cell
 * 
 */
fn trap(heights: &[i32]) -> i32 {
    let (mut left, mut right) = (0,heights.len()-1);
    let mut water = 0;
    let (mut max_left, mut max_right) = (0,0);

    while(left < right) {
        //process left if cell is lower than current right
        if heights[left] < heights[right] {
            //current cell higher than local max
            if heights[left] > max_left {
                max_left = heights[left];
            } else {
                //commit water (difference between) max and left
                water += max_left - heights[left];
            }
            left += 1;  //increment left pointer
        } else {
            if heights[right] > max_right {
                max_right = heights[right];
            } else {
                water += max_right - heights[right];
            }
            right-= 1 //decrement right pointer;
        }
    };

    water
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestCase {
        input: Vec<i32>,
        output: i32
    }

    impl TestCase {
        fn new(i: Vec<i32>,o: i32) -> Self {
            TestCase { input: (i), output: (o) }
        }
    }

    static TO_TEST: [(&str,fn(&[i32])->i32);2] = [
            ("2_PASS",trap_2_pass),
            ("1_PASS",trap)
    ];

    #[test]
    fn test_all() {

        let cases: [TestCase;2] = [
            TestCase::new(vec![0,1,0,2,1,0,1,3,2,1,2,1],6),
            TestCase::new(vec![4,2,0,3,2,5],9)
        ];

        for (s,f) in TO_TEST {
            for c in cases.iter() {
                assert_eq!(
                    c.output,
                    f(&c.input),
                    "Failed on implementation {s:?}"
                )
            }
        }

    }
}