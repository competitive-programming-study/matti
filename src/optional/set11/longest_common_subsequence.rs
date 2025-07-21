///
/// ## Longest Common subsequence length
///
/// Given 2 strings, s0,s1 we have to compute the longest common subsequence length
///
/// A common subsequence is an ordered sequence of characters that appears in both strings
///
/// ## Returns
/// The length as an `usize` of the longest common subsequence
///
/// ## Strategy
/// This is a common dynamic programming problem that uses a memoization matrix to
/// keep the longest common subsequence for all couples of substrings in the 2 strings.
/// This memoization allows us to adopt a bottom-up matrix, composing new results by older
/// results. This techniques acts in contraposition to divide-and-conquer that would compute
/// many more extra steps by solving the same subproblems multiple times
///
/// ## Complexity
/// All dynamic programming techniques use some kind of memoization to store subproblem results
/// In this case we use a memoization matrix. So space complexity is `O(n * m)` in respect to both
/// string lengths
///
/// Time complexity is O(n*m) since all cells of the memoization matrix have to be compiled, in this
/// case computing a result requires in extra steps that amoumt to constant time.
///
///
pub fn longest_common_subsequence_len(s1: &str, s2: &str) -> usize {
    //construct the memoization matrix as a 0-filled (n)*(m) matrix
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; s2.len() + 1]; s1.len() + 1];
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();

    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            matrix[i][j] = if s1[i - 1] == s2[j - 1] {
                1 + matrix[i - 1][j - 1]
            } else {
                matrix[i - 1][j].max(matrix[i][j - 1])
            }
        }
    }
    matrix[s1.len()][s2.len()]
}
