#![allow(unused)]

/**
 * ARRAY LEADERS
 *
 * You are given an array arr of positive integers.
 * Your task is to find all the leaders in the array.
 * An element is considered a leader if it is greater
 * than or equal to all elements to its right.
 * The rightmost element is always a leader.
 *
 * @par: reference to a slice of u32s
 * @returns: a Vec<u32> with the leaders if the par is not-empty
 *
 * Since we inspect the slice one time, plus the k/2 iteration to reverse the
 * leaders vector in-place, the time complexity is:
 *
 * T = n + (k<=n)/2 = O(n)
 * S = O(1)
 *
 */
pub fn array_leaders(arr: &[u32]) -> Option<Vec<u32>> {
    let mut leaders: Vec<u32>;
    let mut max;
    match arr.last() {
        Some(v) => {
            max = *v;
            leaders = vec![max];
        }
        None => return None,
    };

    for e in arr.iter().rev().skip(1) {
        if *e >= max {
            max = *e;
            leaders.push(max);
        };
    }

    leaders.reverse();

    Some(leaders)
}

#[cfg(test)]
mod test {
    use super::array_leaders;

    struct TestCase {
        input: Vec<u32>,
        output: Option<Vec<u32>>,
    }

    impl TestCase {
        fn new(i: Vec<u32>, o: Option<Vec<u32>>) -> Self {
            TestCase {
                input: (i),
                output: (o),
            }
        }
    }

    #[test]
    fn test_all_cases() {
        let to_test = [
            TestCase::new(vec![], None),
            TestCase::new(vec![16, 17, 4, 3, 5, 2], Some(vec![17, 5, 2])),
            TestCase::new(vec![10, 4, 2, 4, 1], Some(vec![10, 4, 4, 1])),
            TestCase::new(vec![5, 10, 20, 40], Some(vec![40])),
            TestCase::new(vec![30, 10, 10, 5], Some(vec![30, 10, 10, 5])),
        ];

        for e in to_test {
            assert_eq!(array_leaders(&e.input), e.output)
        }
    }
}
