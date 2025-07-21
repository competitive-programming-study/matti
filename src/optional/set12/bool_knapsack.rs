///
/// ## Boolean Knapsack problem
///
/// Given an array of object, each one with a weight and a value
/// return the maximum value reachable not going higher than a
/// specified capacity b
///
/// # Returns
/// An usize that specifies the max value achievable
///
/// ## Strategy
/// A bruteforce approach would be exponential in the number of objects
/// while no greedy algorithms exist with this specification of the problem
///
/// We use a dynamic programming approach using a memoization matrix n * b
///
/// The first row and colums are initialized to 0 since the value with 0 object
/// is 0 and no object can be choosen with a capacity of 0.
///
/// For each cell of the matrix we choose:
/// - propagate the value from the cell before if the weitgh of the object is higher
///   than the capacity
/// - taking the max value between the previous cell or the cell in the row W - wt
///
/// In this approach for every object we always choose either to take them or not
/// based on the value maximization
///
/// ## Complexity
/// Same as most dynamic progamming that involve a matrix the complexity
/// (both space and time) is O(n*W) where n is teh number of objects and W is the capacity
///
pub fn bool_knapsack(objects: &[(usize, usize)], capacity: usize) -> usize {
    let mut mem_matrix = vec![vec![0; capacity + 1]; objects.len() + 1];

    for i in 1..=objects.len() {
        for w in 0..=capacity {
            let ob = objects[i - 1];
            let (v, wt) = (ob.0, ob.1);

            //if the object weight is greater than the capacity
            if wt > w {
                mem_matrix[i][w] = mem_matrix[i - 1][w]
            } else {
                //not take the item or take it
                mem_matrix[i][w] = Ord::max(mem_matrix[i - 1][w], mem_matrix[i - 1][w - wt] + v);
            }
        }
    }

    mem_matrix[objects.len()][capacity]
}
