extern crate __procon_math_traits as math_traits;

use std::{
    marker::PhantomData,
    ops::{Add, Bound, RangeBounds},
    usize,
};

use math_traits::{BoundedAbove, BoundedBelow, Maximum, Minimum, Monoid, Zero};

pub trait MapMonoid {
    type M: Monoid;
    type F: Clone;

    fn id() -> <Self::M as Monoid>::T {
        Self::M::id()
    }

    fn op(a: &<Self::M as Monoid>::T, b: &<Self::M as Monoid>::T) -> <Self::M as Monoid>::T {
        Self::M::op(a, b)
    }

    fn map_id() -> Self::F;
    fn map(f: &Self::F, x: &<Self::M as Monoid>::T) -> <Self::M as Monoid>::T;
    fn composite(f: &Self::F, g: &Self::F) -> Self::F;
}

pub struct LazySegTree<Map: MapMonoid> {
    n: usize,
    size: usize,
    log: usize,
    data: Vec<<Map::M as Monoid>::T>,
    lazy: Vec<Map::F>,
}

impl<Map: MapMonoid> From<Vec<<Map::M as Monoid>::T>> for LazySegTree<Map> {
    fn from(v: Vec<<Map::M as Monoid>::T>) -> Self {
        let n = v.len();
        let size = v.len();
        let size = size.next_power_of_two();
        let log = {
            let mut v = 0;
            let mut sz = size;
            while sz > 0 {
                sz >>= 1;
                v += 1;
            }
            v
        };
        let mut data = vec![Map::id(); 2 * size];
        data[size..size + v.len()].clone_from_slice(&v);
        let lazy = vec![Map::map_id(); size];
        let mut ret = Self {
            n,
            size,
            log,
            data,
            lazy,
        };
        for i in (1..size).rev() {
            ret.update(i);
        }
        ret
    }
}

impl<Map: MapMonoid> LazySegTree<Map> {
    pub fn new(size: usize) -> Self {
        vec![Map::id(); size].into()
    }

    pub fn get(&mut self, mut idx: usize) -> <Map::M as Monoid>::T {
        idx += self.size;
        for i in (1..=self.log).rev() {
            self.push(idx >> i);
        }
        self.data[idx].clone()
    }

    pub fn set(&mut self, mut idx: usize, v: <Map::M as Monoid>::T) {
        idx += self.size;
        for i in (1..=self.log).rev() {
            self.push(idx >> i);
        }
        self.data[idx] = v;
        for i in 1..=self.log {
            self.update(idx >> i);
        }
    }

    pub fn query<R>(&mut self, range: R) -> <Map::M as Monoid>::T
    where
        R: RangeBounds<usize>,
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

    pub fn apply_at(&mut self, mut idx: usize, f: Map::F) {
        idx += self.size;
        for i in (1..=self.log).rev() {
            self.push(idx >> i);
        }
        self.data[idx] = Map::map(&f, &self.data[idx]);
        for i in 1..=self.log {
            self.update(idx >> i);
        }
    }

    pub fn apply_range<R>(&mut self, range: R, f: Map::F)
    where
        R: RangeBounds<usize>,
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
        self.apply_range_inner(start, end, f)
    }
}

/// Do-While macro
macro_rules! do_ {
    {$body: block while $cond: expr} => {
        while { $body; $cond } {}
    }
}

impl<Map: MapMonoid> LazySegTree<Map> {
    pub fn max_right<G>(&mut self, mut l: usize, g: G) -> usize
    where
        G: Fn(<Map::M as Monoid>::T) -> bool,
    {
        assert!(g(Map::id()));
        assert!(l <= self.n);
        if l == self.n {
            return self.n;
        }
        l += self.size;
        for i in (1..=self.log).rev() {
            self.push(l >> i);
        }
        let mut mv = Map::id();
        do_! ({
            while l % 2 == 0 {
                l >>= 1;
            }
            if !g(Map::op(&mv, &self.data[l])) {
                while l < self.size {
                    self.push(l);
                    l *= 2;
                    let res = Map::op(&mv, &self.data[l]);
                    if g(res.clone()) {
                        mv = res;
                        l += 1;
                    }
                }
                return l - self.size;
            }
            mv = Map::op(&mv, &self.data[l]);
            l += 1;
        } while {
            let l = l as isize;
            (l & -l) != l
        });
        self.n
    }

    pub fn min_left<G>(&mut self, mut r: usize, g: G) -> usize
    where
        G: Fn(<Map::M as Monoid>::T) -> bool,
    {
        assert!(r <= self.n);
        assert!(g(Map::id()));
        if r == 0 {
            return 0;
        }
        r += self.size;
        for i in (1..=self.log).rev() {
            self.push((r - 1) >> i);
        }
        let mut mv = Map::id();
        do_! ({
            r -= 1;
            while r % 2 == 0 {
                r >>= 1;
            }
            if !g(Map::op(&self.data[r], &mv)) {
                while r < self.size {
                    self.push(r);
                    r = 2 * r + 1;
                    let res = Map::op(&self.data[r], &mv);
                    if g(res.clone()) {
                        mv = res;
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            mv = Map::op(&self.data[r], &mv);
        } while {
            let r = r as isize;
            (r & -r) != r
        });
        self.n
    }
}

impl<Map: MapMonoid> LazySegTree<Map> {
    fn push(&mut self, idx: usize) {
        self.apply(2 * idx, self.lazy[idx].clone());
        self.apply(2 * idx + 1, self.lazy[idx].clone());
        self.lazy[idx] = Map::map_id();
    }

    fn apply(&mut self, idx: usize, f: Map::F) {
        self.data[idx] = Map::map(&f, &self.data[idx]);
        if idx < self.size {
            self.lazy[idx] = Map::composite(&f, &self.lazy[idx]);
        }
    }

    fn update(&mut self, idx: usize) {
        self.data[idx] = Map::op(&self.data[2 * idx], &self.data[2 * idx + 1]);
    }

    fn query_inner(&mut self, mut l: usize, mut r: usize) -> <Map::M as Monoid>::T {
        assert!(l <= r);
        if l == r {
            return Map::id();
        }
        l += self.size;
        r += self.size;
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push(r >> i);
            }
        }
        let mut vl = Map::id();
        let mut vr = Map::id();
        while l < r {
            if l & 1 > 0 {
                vl = Map::op(&vl, &self.data[l]);
                l += 1;
            }
            if r & 1 > 0 {
                r -= 1;
                vr = Map::op(&self.data[r], &vr);
            }
            l >>= 1;
            r >>= 1;
        }
        Map::op(&vl, &vr)
    }

    fn apply_range_inner(&mut self, mut l: usize, mut r: usize, f: Map::F) {
        assert!(l <= r);
        if l == r {
            return;
        }
        l += self.size;
        r += self.size;
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
        {
            let l2 = l;
            let r2 = r;
            while l < r {
                if l & 1 > 0 {
                    self.apply(l, f.clone());
                    l += 1;
                }
                if r & 1 > 0 {
                    r -= 1;
                    self.apply(r, f.clone());
                }
                l >>= 1;
                r >>= 1;
            }
            l = l2;
            r = r2;
        }
        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
    }
}

pub struct MaxAdd<T>(PhantomData<fn() -> T>);
impl<T> MapMonoid for MaxAdd<T>
where
    T: Clone + Eq + Ord + BoundedBelow + Zero + Add<Output = T>,
{
    type M = Maximum<T>;
    type F = T;

    fn map_id() -> Self::F {
        T::zero()
    }

    fn map(f: &Self::F, x: &<Self::M as Monoid>::T) -> <Self::M as Monoid>::T {
        f.clone() + x.clone()
    }

    fn composite(f: &Self::F, g: &Self::F) -> Self::F {
        f.clone() + g.clone()
    }
}

pub struct MinAdd<T>(PhantomData<fn() -> T>);
impl<T> MapMonoid for MinAdd<T>
where
    T: Clone + Eq + Ord + BoundedAbove + Zero + Add<Output = T>,
{
    type M = Minimum<T>;
    type F = T;

    fn map_id() -> Self::F {
        T::zero()
    }

    fn map(f: &Self::F, x: &<Self::M as Monoid>::T) -> <Self::M as Monoid>::T {
        f.clone() + x.clone()
    }

    fn composite(f: &Self::F, g: &Self::F) -> Self::F {
        f.clone() + g.clone()
    }
}

use std::fmt::{Debug, Error, Formatter, Write};
impl<Map> Debug for LazySegTree<Map>
where
    Map: MapMonoid,
    Map::F: Debug,
    <Map::M as Monoid>::T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for i in 0..self.log - 1 {
            for j in 0..1 << i {
                f.write_fmt(format_args!(
                    "{:?}[{:?}]\t",
                    self.data[(1 << i) + j],
                    self.lazy[(1 << i) + j]
                ))?;
            }
            f.write_char('\n')?;
        }
        for i in 0..self.size {
            f.write_fmt(format_args!("{:?}\t", self.data[self.size + i]))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy_segtree_range_add_range_maximum() {
        let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let mut seg: LazySegTree<MaxAdd<_>> = v.clone().into();
        eprintln!("{:?}", seg);
        test_all_query(&v, &mut seg);

        change_range_value(1, 3, 6, &mut v, &mut seg);
        eprintln!("{:?}", seg);
        test_all_query(&v, &mut seg);

        eprintln!("{:?}", seg);
        change_range_value(3, 7, 2, &mut v, &mut seg);
    }

    fn change_range_value(
        l: usize,
        r: usize,
        value: i32,
        v: &mut [i32],
        seg: &mut LazySegTree<MaxAdd<i32>>,
    ) {
        (l..=r).for_each(|i| v[i] += value);
        seg.apply_range(l..=r, value);
    }

    fn test_all_query(v: &[i32], seg: &mut LazySegTree<MaxAdd<i32>>) {
        for l in 0..v.len() - 1 {
            for r in l + 1..v.len() {
                let ans = v[l..=r].iter().max().copied().unwrap();
                assert_eq!(ans, seg.query(l..=r), "({}..={})", l, r);
            }
        }
    }
}
