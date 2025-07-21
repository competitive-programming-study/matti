use crate::data_structs::fenwick_tree::FenwickTree;
use std::ops::{Add,Sub,Neg};

pub struct UpdateArray<T> {
    ft: FenwickTree<T>,
}

impl<T> UpdateArray<T>
where
    T: Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Clone + Copy + Default,
{
    pub fn with_len(n: usize, sum_invariant: T) -> Self {
        Self {
            ft: FenwickTree::with_len(n, sum_invariant),
        }
    }

    pub fn access(&self, i: usize) -> Result<T, &str> {
        self.ft.sum(i)
    }

    ///## Range Update
    ///
    /// We add a value to the first item in the range and we subtract the same
    /// value to the first item after the range
    pub fn range_update(&mut self, l: usize, r: usize, v: T) -> Result<(), &str> {
        let ln = self.ft.len();

        if l > r {
            return Err("Left index out of range");
        } else if r >= ln {
            return Err("Right index out of bounds");
        };

        let _ = self.ft.add(l, v);
        if r + 1 < ln {
            let _ = self.ft.add(r, -v);
        }
        Ok(())
    }

    ///## Add
    /// Add is just a special case of range_update where left == right
    pub fn add(&mut self, i: usize, v: T) -> Result<(), &str> {
        self.range_update(i, i, v)
    }
}