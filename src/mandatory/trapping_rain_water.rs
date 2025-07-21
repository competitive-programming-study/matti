/// # Trapping Rain Water
///
/// Given an array of positive integers `u32` that represent heights find the max amount
/// of water that can be trapped between all heights
///
/// ## Example
/// ```text
/// Height: [0,1,0,2,1,0,1,3,2,1,2,1]
///
///             X
///       X w w X X w X
///   X w X w X X X X X X
///  XXXXXXXXXXXXXXXXXXXX
/// ```
/// ## Returns
/// The amount of water contained between heights as an `usize`
///
/// ## Strategy
///
/// We use a 2 pointers approach, iterating through the heights conditionally from
/// left or right based on the lowest height. For each side we keep 2 maxima and
/// stop when the pointers flip.
///
/// ## Complexity
/// We require no additional space while solving the problem in O(n) time
pub fn trapping_rain_water(heights: &[u32]) -> usize {
    if heights.is_empty() {
        return 0;
    }

    let mut water = 0;
    let (mut max_left, mut max_right) = (0, 0);
    let (mut left, mut right) = (0, heights.len() - 1);

    //Iterate until the pointers flip
    while left < right {
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
        } else {
            //Same thing as above
            if heights[right] > max_right {
                max_right = heights[right];
            } else {
                water += max_right - heights[right];
            }

            right -= 1 //decrement right pointer;
        }
    }

    water as usize
}

#[cfg(test)]
mod test_trapping_rain_water {
    use super::trapping_rain_water;

    #[test]
    fn test_empty() {
        for i in 0..=10 {
            for j in 0..=10 {
                assert_eq!(trapping_rain_water(&vec![i; j]), 0);
            }
        }
    }

    #[test]
    fn test_1() {
        let heights = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let expected = 6;
        assert_eq!(trapping_rain_water(&heights), expected);
    }

    #[test]
    fn test_2() {
        let heights = [4, 2, 0, 3, 2, 5];
        let expected = 9;
        assert_eq!(trapping_rain_water(&heights), expected);
    }
}
