#![allow(unused)]

/// **TRAP WATER (2 passes)**
/// 
/// Given an array of positive integers `u32` that represent heights find the max amount
/// of water that can be trapped between all heights
/// 
/// **EXAMPLE**
/// ```text
/// Height= [0,1,0,2,1,0,1,3,2,1,2,1]
/// 
///             X
///       X w w X X w X
///   X w X w X X X X X X
///  XXXXXXXXXXXXXXXXXXXX
/// ```
/// 
/// 
/// A simple approach consists is comparing the current height to
/// a local maximum. if it's lower then we accumulate the difference
/// between those 2, in a `local water` variable (expecting a higher max)
/// committing it to the global water, when we find it.
/// 
/// **IMPORTAT**: This approach is subsceptible to local maxima, where we stop
/// accumulating the water. To solve this, we iterate 2 times, one from left to
/// right, the other from right to left (stopping at the local maxima).
/// 
/// *Time Complexity*: 2n ~ O(n)
/// 
/// *Space Complexity*: O(1)
pub fn trap_water_2_pass(heights: &[u32]) -> u32 {
    //used for the second iteration
    let mut max_height_idx = 0;

    let (mut max_height, mut local_water, mut water) = (0, 0, 0);

    //First iteration
    for (i, &h) in heights.iter().enumerate() {
        //Commit the water if found an higher max
        if h >= max_height {
            max_height = h;
            max_height_idx = i;
            water += local_water;
            local_water = 0;    //reset the local to 0
        } else {
            //Accumulate the difference
            local_water += max_height - h;
        }
    }

    (max_height, local_water) = (0, 0);

    // Second iteration (right to left) until local maxima
    for &e in heights.iter().skip(max_height_idx).rev() {
        //Commit to global water
        if e >= max_height {
            water += local_water;
            max_height = e;
            local_water = 0;
        } else {
            //accumulate the difference
            local_water += max_height - e;
        }
    }

    water
}

/// **TRAP WATER (2 passes)**
/// 
/// Given an array of positive integers `u32` that represent heights find the max amount
/// of water that can be trapped between all heights
/// 
/// **EXAMPLE**
/// ```text
/// Height: [0,1,0,2,1,0,1,3,2,1,2,1]
/// 
///             X
///       X w w X X w X
///   X w X w X X X X X X
///  XXXXXXXXXXXXXXXXXXXX
/// ```
/// 
/// 
/// We use a 2 pointers approach, iterating through the heights conditionally from
/// left or right based on the lowest height. For each side we keep 2 maxima and
/// stop when the pointers flip.
/// 
/// **IMPORTANT**: In this way we manage to only iterate through the vector one time,
/// It's important to note that a 2 iteration approach may be faster because it better
/// uses spacial locality hence maximize cache performance
/// 
/// *Time Complexity*: O(n)
/// 
/// *Space Complexity*: O(1)
pub fn trap_water(heights: &[u32]) -> u32 {
    let (mut left, mut right) = (0, heights.len() - 1);
    let mut water = 0;
    let (mut max_left, mut max_right) = (0, 0);

    //Iterate until the pointers flip
    while (left < right) {
        //Conditionally process left or right based on the current height
        
        if heights[left] < heights[right] {

            //found new max (commit to global water)
            if heights[left] > max_left {
                max_left = heights[left];
            } else {
                //accumulate difference between old max and current height
                water += max_left - heights[left];
            }
            left += 1; //increment left pointer
        
        } else {    //Same thing as above
            
            if heights[right] > max_right {
                max_right = heights[right];
            } else {
                water += max_right - heights[right];
            }

            right -= 1 //decrement right pointer;
        }
    }

    water
}
