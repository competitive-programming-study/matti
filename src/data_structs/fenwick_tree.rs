use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug)]
pub struct FenwickTree<T> {
    tree: Vec<T>,
}

impl<T> FenwickTree<T>
where
    T: Add<Output = T> + Sub<Output = T> + Clone + Copy + Default,
{
    pub fn with_len(n: usize, sum_invariant: T) -> Self {
        Self {
            tree: vec![sum_invariant; n + 1], //account for the 0 indexing
        }
    }

    pub fn from(slice: &[T]) -> Self {
        let mut v = Vec::with_capacity(slice.len() + 1);
        v.push(T::default()); //we'll never use it
        if slice.len() > 0 {
            v.push(slice[0]);
            for i in 1..slice.len() {
                v.push(slice[i] + v[i - 1])
            }
        }
        Self { tree: v }
    }

    pub fn len(&self) -> usize {
        self.tree.len() - 1 //account for the 0 indexing
    }

    pub fn add(&mut self, i: usize, delta: T) -> Result<(), &str> {
        let mut i = i + 1;
        if i >= self.tree.len() {
            return Err("Index overflow");
        }
        while i < self.tree.len() {
            self.tree[i] = self.tree[i] + delta;
            i = Self::next_sibling(i);
        }
        Ok(())
    }

    pub fn access(&self, i: usize) -> Result<T, &str> {
        let i = i + 1;
        Ok(if i == 0 {
            self.tree[0]
        } else {
            self.sum(i)? - self.sum(i - 1)?
        })
    }

    pub fn sum(&self, i: usize) -> Result<T, &str> {
        let mut i = i + 1;
        if i >= self.tree.len() {
            return Err("Index overflow");
        }
        let mut sum: T = self.tree[i];
        i = Self::parent(i);
        while i != 0 {
            sum = sum + self.tree[i];
            i = Self::parent(i);
        }

        Ok(sum)
    }

    pub fn range_sum(&self, left: usize, right: usize) -> Result<T, &str> {
        let sum = self.sum(right)?;
        if left == 0 {
            Ok(sum)
        } else {
            Ok(sum - self.sum(left - 1)?)
        }
    }

    fn isolate_trailing_one(i: usize) -> usize {
        if i == 0 { 0 } else { 1 << i.trailing_zeros() }
    }

    #[allow(unused)]
    fn isolate_trailing_one_explicit(i: usize) -> usize {
        i & (!i + 1) //(!i + 1) is the same as -i (2's complement)
    }

    fn parent(i: usize) -> usize {
        i - Self::isolate_trailing_one(i)
    }

    fn next_sibling(i: usize) -> usize {
        i + Self::isolate_trailing_one(i)
    }
}

#[derive(Debug)]
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
        Ok(self.ft.sum(i)?)
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
        Ok(self.range_update(i, i, v)?)
    }
}

pub struct RangeUpdate<T> {
    ft1: FenwickTree<T>,
    ft2: FenwickTree<T>,
}

impl<T> RangeUpdate<T>
where
    T: Neg<Output = T>
        + Sub<Output = T>
        + Add<Output = T>
        + Mul<usize, Output = T>
        + Clone
        + Copy
        + Default,
{
    pub fn with_len(n: usize, sum_invariant: T) -> Self {
        Self {
            ft1: FenwickTree::with_len(n, sum_invariant),
            ft2: FenwickTree::with_len(n, sum_invariant),
        }
    }

    pub fn access(&self, i: usize) -> Result<T, &str> {
        self.ft1.access(i)
    }

    pub fn sum(&self, i: usize) -> Result<T, &str> {
        let l = self.ft1.sum(i)?;
        let r = self.ft2.sum(i)?;
        Ok(l + r)
    }

    fn multiply(v: T, i: usize) -> T {
        let mut v1 = v;
        for _ in 1..i {
            v1 = v1 + v
        }
        v1
    }

    pub fn range_update(&mut self, l: usize, r: usize, v: T) -> Result<(), &str> {
        if l > r {
            return Err("Left index out of range");
        } else if r >= self.ft1.len() {
            return Err("Right Index out of bounds");
        }

        let _ = self.ft1.add(l, v);

        //add the error correction term
        let _ = self.ft2.add(l, -v * (l - 1));

        if (r + 1) < self.ft1.len() {
            let _ = self.ft1.add(r + 1, -v);
            let _ = self.ft2.add(r + 1, RangeUpdate::multiply(v, r))?;
        }

        Ok(())
    }
}
