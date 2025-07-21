use std::cmp::Ordering::*;
use std::collections::HashMap;
pub struct Query {
    left: usize,
    right: usize,
    idx: usize,
}

/// # Power Array
///
/// Given a list of integers we have to handle a set of offline queries of the form
/// of `power(l,r)`. For each power query in a specific interval `[l,r]` we have to
/// return the sum of all items multiplied by their respective occurrences squared.
///
/// For each distinct interval we have to compute: `sum_(k = l)^r  a[k] * O(a[k])^2`.
///
/// ## Parameter
/// each query is a tuple of intervals of the form `(usize,suzie)` denoting
/// left and right respectively.
///
/// ## Returns
/// a vector of powers sorted for each query
///
/// ## Strategy
/// Since queries are known in advance, we can precompute the occurrence of each item of of the
/// query range `[ql,qr]`. We can store the occurrence in a frequency map, an array map would have
/// been suitable but we can achieve better performances if the range of items is low.
///
/// After a query has been performed, we can use the occurrence count of the last range to compute (at least in part)
/// the new query range. So we evict items that do not appear in this range and add the new ones. If ranges overlap
/// this is a neat optimization.
///
/// Since the amortized complexity is strictly related to the order of queries, we cna reorder them using Mo's technique,
/// so store them in sqrt(n) buckets based on their left range point, and sort the buckets by right range points. We also
/// have to store the index to compose them back.
///
/// ## Complexity
/// Since the left pointer can move at most sqrt(n) positions and the left pointer at most n positions the asymptotic complexity
/// is O(sqrt(n)(q + n))
///
/// The space complexity is O(n)
///
pub fn powerful_array(a: &[i64], q: &[(usize, usize)]) -> Vec<i64> {
    let mut answers = vec![0; q.len()];
    let mut queries: Vec<Query> = q
        .iter()
        .enumerate()
        .map(|(idx, &(left, right))| Query { left, right, idx })
        .collect();
    let block_size = (a.len() as f64).sqrt() as usize;

    //we sort the queries based on the bucket index (left index/block_size) and right index
    queries.sort_by(
        |a, b| match (a.left / block_size).cmp(&((b.left) / block_size)) {
            Equal => a.right.cmp(&b.right),
            Less => Less,
            Greater => Greater,
        },
    );

    let mut frequency: HashMap<i64, i64> = HashMap::new();
    let (mut left, mut right) = (0, 0);
    let mut power = a[0];
    frequency.insert(a[0], 1);

    //closures to update power and frequency map

    /*
     * We update power, using a delta:
     *
     * After an insertion we have
     * ((c+1)^2 * x) - (c^2 * x) = (2c + 1) * x
     *
     */
    fn add(x: i64, frequency: &mut HashMap<i64, i64>, power: &mut i64) {
        let entry = frequency.entry(x).or_insert(0);
        let count = *entry;
        *power += (2 * count + 1) * x;
        *entry += 1;
    }

    fn rem(x: i64, frequency: &mut HashMap<i64, i64>, power: &mut i64) {
        let entry = frequency.get_mut(&x).unwrap(); // guaranteed present
        let count = *entry;
        *power -= (2 * count - 1) * x;
        *entry -= 1;
    }

    for q in queries {
        while left > q.left {
            left -= 1;
            add(a[left], &mut frequency, &mut power);
        }

        while left < q.left {
            left += 1;
            rem(a[right], &mut frequency, &mut power);
        }

        while right < q.right {
            right += 1;
            add(a[right], &mut frequency, &mut power);
        }

        while right > q.right {
            right -= 1;
            rem(a[right], &mut frequency, &mut power);
        }

        answers[q.idx] = power;
    }

    answers
}

#[cfg(test)]
mod test_powerful_array {
    use super::*;

    #[test]
    fn test_codeforces() {
        //this test case considers 1-indexing
        assert_eq!(
            powerful_array(&[i64::MAX, 1, 2, 1], &[(1, 2), (1, 3)]),
            vec![3, 6]
        );
        //this test case considers 0-indexing
        assert_eq!(
            powerful_array(&[1, 1, 2, 2, 1, 3, 1, 1], &[(2, 7), (1, 6), (2, 7)]),
            vec![20, 20, 20]
        );
    }
}
