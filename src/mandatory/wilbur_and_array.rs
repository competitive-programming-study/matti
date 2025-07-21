///
/// # Wilbur and Array
///
/// Given a target array, we want to know how many steps it requires to get to that
/// array starting from an array of zeroes, for each step we can:
/// - add 1 to all elements after an index i
/// - subtract 1 to all elements before an index i
///
/// ## Returns
/// The number of steps as an usize
///
/// ## Strategy
/// We use the amount of change between two consecutive positions as a step increment.
///
/// For each item we store the previous, we start storing 0, and we increment the step
/// as the absolute difference between the item and the previous
///
pub fn wilbur_array(target: &[isize]) -> usize {
    let (mut steps, mut prev) = (0, 0);

    for &x in target {
        steps += (x - prev).abs();
        prev = x;
    }

    steps as usize
}

#[cfg(test)]
mod test_wilbur_array {
    use super::*;

    #[test]
    fn test_empty() {
        let target: &[isize] = &[];
        assert_eq!(wilbur_array(target), 0);
    }

    #[test]
    fn test_single_element() {
        let target = &[5];
        assert_eq!(wilbur_array(target), 5);
    }

    #[test]
    fn test_multiple_elements() {
        let target = &[1, 4, 2, 8];
        assert_eq!(wilbur_array(target), 12);
    }

    #[test]
    fn test_all_elements_same() {
        let target = &[3, 3, 3, 3];
        assert_eq!(wilbur_array(target), 3);
    }

    #[test]
    fn test_negative_numbers() {
        let target = &[-3, -1, -7, -4];
        assert_eq!(wilbur_array(target), 14);
    }
}
