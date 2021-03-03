extern crate __procon_math_traits as math_traits;

use math_traits::Monoid;

use std::ops::{RangeBounds, Bound};

pub struct SegTree<M>
where
    M: Monoid,
{
    size: usize,
    log: usize,
    data: Vec<M::T>,
}

impl<M: Monoid> From<Vec<M::T>> for SegTree<M> {
    fn from(v: Vec<M::T>) -> Self {
        let mut size = 1;
        let mut log = 0;
        while v.len() > size { size <<= 1; log += 1; }
        let mut data = vec![M::id(); 2*size];
        data[size..(size+v.len())].clone_from_slice(&v);
        let mut st = Self { size, log, data };
        (0..size).rev().for_each(|i| st.update(i));
        st
    }
}

impl<M: Monoid> SegTree<M> {
    pub fn new(size: usize) -> SegTree<M> {
        vec![M::id(); size].into()
    }

    pub fn get(&self, idx: usize) -> M::T {
        self.data[self.size + idx].clone()
    }

    pub fn set(&mut self, mut idx: usize, v: M::T) {
        idx += self.size;
        self.data[idx] = v;
        (0..=self.log).for_each(|i| self.update(idx >> i));
    }

    fn update(&mut self, idx: usize) {
        self.data[idx] = M::op(&self.data[idx * 2], &self.data[idx * 2 + 1]);
    }

    fn query_inner(&self, mut l: usize, mut r: usize) -> M::T {
        assert!(l <= r);
        let mut ret = M::id();
        l += self.size;
        r += self.size;
        while l < r {
            if (l & 1) > 0 {
                ret = M::op(&ret, &self.data[l]);
                l += 1;
            }
            if (r & 1) > 0 {
                ret = M::op(&ret, &self.data[r - 1]);
            }
            l >>= 1;
            r >>= 1;
        }
        ret
    }

    pub fn query<R>(&self, range: R) -> M::T
    where
        R: RangeBounds<usize>
    {
        use Bound::*;
        let start = match range.start_bound() {
            Unbounded => 0,
            Included(&i) => i,
            Excluded(&i) => i + 1,
        };
        let end = match range.end_bound() {
            Unbounded => self.size,
            Included(&i) => i + 1,
            Excluded(&i) => i,
        };
        self.query_inner(start, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::math_traits::*;

    #[test]
    fn test_sum_segtree() {
        let v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let st: SegTree<Additive<_>> = v.clone().into();
        check_all_sum(&st, &v);
    }

    fn check_all_sum<M: Monoid>(st: &SegTree<M>, v: &[M::T])
    where
        M::T: PrimitiveInteger
    {
        let n = v.len();
        for l in 0..n-1 {
            for r in l+1..n {
                let ans = v[l..=r].iter().fold(M::T::zero(), |a, &b| a + b);
                assert_eq!(ans, st.query(l..=r));
            }
        }
    }
}