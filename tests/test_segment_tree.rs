use code::data_structs::segment_tree::SegmentTree;
use rand::{rngs::StdRng, Rng, SeedableRng};

/* ---------- helpers ---------- */

/// Slow but simple reference implementation for sanity‑checking.
fn naive_min_after_add(
    mut data: Vec<i64>,
    ops: &[(usize, usize, i64)],
    ql: usize,
    qr: usize,
) -> i64 {
    for &(l, r, v) in ops {
        if l <= r {
            for i in l..=r {
                data[i] += v;
            }
        } else {
            for i in l..data.len() {
                data[i] += v;
            }
            for i in 0..=r {
                data[i] += v;
            }
        }
    }
    let slice = if ql <= qr {
        &data[ql..=qr]
    } else {
        // wrap‑around query
        let min1 = data[ql..].iter().min().unwrap();
        let min2 = data[..=qr].iter().min().unwrap();
        return *min1.min(min2);
    };
    *slice.iter().min().unwrap()
}

fn expect_tree_matches_array(tree: &mut SegmentTree, arr: &[i64]) {
    for i in 0..arr.len() {
        assert_eq!(tree.min_range(i, i), arr[i],
                    "leaf mismatch at index {i}");
    }
}

/* ---------- deterministic unit tests ---------- */

#[test]
fn build_and_point_queries() {
    let data = vec![3, 1, 4, 1, 5, 9, 2];
    let mut st = SegmentTree::build(&data);
    assert_eq!(st.len(), data.len());
    expect_tree_matches_array(&mut st, &data);

    // global min should match std::iter::min
    assert_eq!(st.min_range(0, data.len() - 1),
                *data.iter().min().unwrap());
}

#[test]
fn range_add_then_query() {
    let mut st = SegmentTree::build(&[0, 0, 0, 0]);
    st.add_range(1, 3, 10);          // [0,10,10,10]
    st.add_range(0, 0, -5);          // [-5,10,10,10]
    assert_eq!(st.min_range(0, 3), -5);
    assert_eq!(st.min_range(1, 2), 10);
    expect_tree_matches_array(&mut st, &[-5, 10, 10, 10]);
}

#[test]
fn wrap_around_update_and_query() {
    let mut st = SegmentTree::build(&[7, 7, 7, 7, 7]);
    st.add_range(3, 1, -2); // affects indices 3,4,0,1
    assert_eq!(st.min_range(4, 2), 5); // wrap query too
    expect_tree_matches_array(&mut st, &[5, 5, 7, 5, 5]);
}

#[test]
fn overlapping_updates_idempotent() {
    let mut st = SegmentTree::build(&[1, 1, 1, 1]);
    st.add_range(0, 3, 3);
    st.add_range(1, 2, -1);
    assert_eq!(st.min_range(0, 3), 3); // [4,3,3,4]
    expect_tree_matches_array(&mut st, &[4, 3, 3, 4]);
}

/* ---------- property‑based randomized test ---------- */

#[test]
fn random_operations_against_naive() {
    const N: usize = 32;
    const OPS: usize = 200;
    let seed = 42; // keep deterministic; change for new scenarios
    let mut rng = StdRng::seed_from_u64(seed);

    let base: Vec<i64> = (0..N).map(|_| rng.random_range(-50..50)).collect();
    let mut st = SegmentTree::build(&base);

    let mut log_ops: Vec<(usize, usize, i64)> = Vec::new();

    for _ in 0..OPS {
        if rng.random_bool(0.6) {
            // range add
            let l = rng.random_range(0..N);
            let r = rng.random_range(0..N);
            let v = rng.random_range(-20..20);
            st.add_range(l, r, v);
            log_ops.push((l, r, v));
        } else {
            // range min query – check against naïve every time
            let l = rng.random_range(0..N);
            let r = rng.random_range(0..N);
            let expected = naive_min_after_add(base.clone(), &log_ops, l, r);
            let got = st.min_range(l, r);
            assert_eq!(got, expected,
                        "mismatch after ops={log_ops:?}, query=({l},{r})");
        }
    }
}

