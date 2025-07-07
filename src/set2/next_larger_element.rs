///
/// **NEXT LARGER ELEMENT**
/// 
/// Given an array of n distinct positive integers (u32), return
/// for each element the next greater element
/// 
/// returns an `Option<Vec<i32>>` which is None if the array was empty
/// 
/// **Example**
/// - Given     `[7,2,5,3,4]`
/// - Returns   `[-1,5,-1,4,-1]`
/// 
/// The idea is to use a Stack data structure (we can use a simple vector, with `push/pop`)
/// and iterate from right to left.
/// 
/// For every element, we evict from the stack all elements that are less or equal 
/// than the current. The top of the stack (if present) will be the next larger element.
/// We extract it, and push the current element to the stack
/// 
/// *Space Complexity*: O(n)
/// 
/// *Time Complexity*: O(n)
///  
pub fn next_larger_element(nums: &[u32]) -> Option<Vec<i32>> {
    if nums.is_empty() {
        return None;
    }

    let mut result: Vec<i32> = vec![-1;nums.len()];
    let mut stack = Vec::<u32>::new();

    for (i,&v) in nums.iter().enumerate().rev() {
        //evict all elements that are less than this one
        while let Some(&x) = stack.last() {
            if x <= v {
                stack.pop();
            } else {
                break;
            }
        }

        if let Some(&x) = stack.last() {
            result[i] = x as i32;
        }

        stack.push(v);
    };

    Some(result)
}