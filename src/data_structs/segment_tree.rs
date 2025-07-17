/**
 * Basic implementation of a segment tree of int64 that can 
 * compute range min queries, and can update values in range
 * by addition
 */

pub struct SegmentTree {
    n: usize,
    ln: usize,
    pub tree: Vec<i64>,
    lazy: Vec<i64>
}

impl SegmentTree {
    pub fn len(&self) -> usize {
        self.n
    }

    fn construct(a: &[i64], seg: &mut[i64],idx:usize,left: usize, right:usize) {
        if left == right {
            seg[idx] = a[left];
            return;
        }
        let mid = (left + right)/2;
        Self::construct(a,seg,idx*2 + 1,left,mid);
        Self::construct(a,seg,idx*2 + 2,mid+1,right);
        seg[idx] = Ord::min(seg[idx*2 + 1],seg[idx*2 + 2]);

    } 

    pub fn build(a: &[i64]) -> Self {
        let n = a.len();
        let mut pow_2 = 1usize;
        while pow_2 < n {
            pow_2 *= 2;
        }
        let ln = (pow_2 * 2) - 1;
        let mut tree = vec![i64::MAX;ln];
        let lazy = vec![0;ln];
        Self::construct(a, &mut tree, 0, 0, n-1);
        Self {
            n,ln,tree,lazy
        }
    }

    /// Push a pending addition down to the children of `idx`.
    fn push(&mut self, idx: usize) {
        //base case
        let base = idx * 2;
        if self.lazy[base] == 0 {
            return;
        } 

        //update the children 
        for child in [base+1,base+2] {
            self.tree[child] += self.lazy[idx];
            self.lazy[child] += self.lazy[idx];
        }
        //reset the lazy count
        self.lazy[idx] = 0;
    }

    ///
    /// Internal range_add update that uses lazy propagation to 
    /// perform subtree update on demand
    /// 
    fn range_add_rec(   &mut self, idx: usize, 
                        left: usize, right: usize, 
                        qleft: usize, qright: usize,
                        val: i64
                    ) -> () {
        //check overlaps
        if qleft > right || qright < left {
            return; //no overlap
        }
        if qleft <= left && qright >= right {
            //total overlap
            self.tree[idx] += val;  //update the tree value
            self.lazy[idx] += val;  //set the lazy update for children
            return;
        }

        //assure the children are updated
        self.push(idx);
        //compute the mid for recursion
        let mid = (left + right) / 2;
        let base = idx * 2;

        //compute recursion on children
        self.range_add_rec(base + 1,left,mid,qleft,qright,val);
        self.range_add_rec(base + 2,  mid+1,right,qleft,qright,val);

        //update the current node based on the new value of children
        self.tree[idx] = Ord::min(self.tree[base + 1], self.tree[base + 2]);
    }


    /// 
    /// Internal range min query that uses lazy propagation ensure consistency
    /// of updates to subtrees on demand
    /// 
    fn range_min_rec(   &mut self, idx: usize,
                        left: usize, right: usize,
                        qleft: usize, qright: usize
                    ) -> i64 {
        if qleft > right || qright < left {
            return i64::MAX;    //no overlap
        }
        else if qleft <= left && qright >= right {
            return self.tree[idx];  //total overlap
        }

        //partial overlap
        self.push(idx); //ensure children are updated
        let mid = (right + left)/2;
        let v_left = self.range_min_rec(idx*2 + 1,left,mid,qleft,qright);
        let v_right = self.range_min_rec(idx*2 + 2, mid+1,right,qleft,qright);
        Ord::min(v_left,v_right)
    }

    /* PUBLIC API */
    pub fn add_range(&mut self,qleft:usize,qright:usize,val:i64) {
        if qleft >= (self.n) || qright >= (self.n) { panic!("Range not supported")}
        if qleft <= qright {
            self.range_add_rec(0,0,self.n - 1, qleft, qright, val)
        } else {
            self.range_add_rec(0, 0, self.n - 1, qleft, self.n - 1, val);
            self.range_add_rec(0,0,self.n - 1,0,qright,val);
        }
    }

    pub fn min_range(&mut self, qleft:usize, qright:usize) -> i64 {
        if qleft >= (self.n) || qright >= (self.n) { panic!("Range not supported")}
        if qleft <= qright {
            self.range_min_rec(0,0,self.n-1, qleft, qright)
        } else {
            Ord::min(
                self.range_min_rec(0, 0, self.n - 1, qleft, self.n - 1),
                self.range_min_rec(0,0,self.n - 1,0,qright)
            )
        }
    }

}