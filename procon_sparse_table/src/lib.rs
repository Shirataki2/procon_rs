use std::{
    marker::PhantomData,
    ops::{Bound, RangeBounds},
};

pub type MinSparseTable<T> = SparseTable<Min<T>>;
pub type MaxSparseTable<T> = SparseTable<Max<T>>;

pub struct SparseTable<Op: Operation> {
    pub data: Vec<Op::T>,
    table: Vec<Vec<usize>>,
    logs: Vec<usize>,
}

pub trait Operation {
    type T: Clone;
    fn compare(a: &Self::T, b: &Self::T) -> bool;
}

#[derive(Debug, Default)]
pub struct Min<T>(PhantomData<fn() -> T>);

impl<T: Clone + Ord> Operation for Min<T> {
    type T = T;
    fn compare(a: &Self::T, b: &Self::T) -> bool {
        a < b
    }
}

#[derive(Debug, Default)]
pub struct Max<T>(PhantomData<fn() -> T>);

impl<T: Clone + Ord> Operation for Max<T> {
    type T = T;
    fn compare(a: &T, b: &T) -> bool {
        a > b
    }
}

impl<Op> SparseTable<Op>
where
    Op: Operation,
    Op::T: Clone,
{
    pub fn new(v: &[Op::T]) -> Self {
        let n = v.len();
        let mut logs = vec![0; n + 1];
        for i in 2..=n {
            logs[i] = logs[i >> 1] + 1;
        }
        let mut table = vec![vec![0; logs[n] + 1]; n];
        for i in 0..n {
            table[i][0] = i;
        }
        for k in (1..n).take_while(|k| (1usize << k) <= n) {
            for i in (0..n).take_while(|i| i + (1usize << k) <= n) {
                let v1 = table[i][k - 1];
                let v2 = table[i + (1 << (k - 1))][k - 1];
                table[i][k] = if Op::compare(&v[v1], &v[v2]) { v1 } else { v2 };
            }
        }
        let data = v.to_vec();
        Self {
            data,
            table,
            logs,
        }
    }

    fn query_inner(&self, left: usize, right: usize) -> usize {
        let d = right - left + 1;
        let k = self.logs[d];
        let (v1, v2) = (self.table[left][k], self.table[right + 1 - (1 << k)][k]);
        if Op::compare(&self.data[v1], &self.data[v2]) {
            v1
        } else {
            v2
        }
    }

    pub fn query<R: RangeBounds<usize>>(&self, range: R) -> usize {
        use Bound::*;
        let l = match range.start_bound() {
            Unbounded => 0,
            Included(&i) => i,
            Excluded(&i) => i + 1,
        };
        let r = match range.end_bound() {
            Unbounded => self.data.len(),
            Included(&i) => i,
            Excluded(&i) => i - 1,
        };
        self.query_inner(l, r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_sparse_table() {
        let v = vec![-7, 4, 8, 1, 6, 7, 10, -1, 0, 4, 9, 11];
        let st = MinSparseTable::new(&v);
        for i in 0..v.len()-1 {
            for j in i..v.len()-1 {
                let m = (i..=j).fold(100, |acc, x| std::cmp::min(acc, v[x]));
                assert_eq!(v[st.query(i..=j)], m);
            }
        }
    }

    #[test]
    fn test_max_sparse_table() {
        let v = vec![-7, 4, 8, 1, 6, 7, 10, -1, 0, 4, 9, 11];
        let st = MaxSparseTable::new(&v);
        for i in 0..v.len()-1 {
            for j in i..v.len()-1 {
                let m = (i..=j).fold(-100, |acc, x| std::cmp::max(acc, v[x]));
                assert_eq!(v[st.query(i..=j)], m);
            }
        }
    }
}
