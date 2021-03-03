pub trait Accumulate: Iterator {
    fn accumulate<T>(self, v0: T, f: fn(&T, &Self::Item) -> T) -> AccumulateItertor<Self, T>
    where
        Self: Sized,
    {
        AccumulateItertor {
            sum: v0,
            func: f,
            iter: self,
        }
    }
}

impl<I: ?Sized> Accumulate for I where I: Iterator {}

pub struct AccumulateItertor<I: Iterator, T> {
    sum: T,
    func: fn(&T, &I::Item) -> T,
    iter: I,
}

impl<I, T> Iterator for AccumulateItertor<I, T>
where
    I: Iterator,
    T: Clone,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|v| {
            let v = (self.func)(&self.sum, &v);
            self.sum = v.clone();
            v
        })
    }
}

pub trait BitBruteForce: Iterator {
    fn bit_brute(self) -> BitBruteForceIterator<Self>
    where
        Self: Sized,
    {
        BitBruteForceIterator {
            vec: self.collect(),
            mask: 0,
        }
    }
}

impl<I: ?Sized> BitBruteForce for I where I: Iterator {}

pub struct BitBruteForceIterator<I: Iterator> {
    vec: Vec<I::Item>,
    mask: usize,
}

impl<I> Iterator for BitBruteForceIterator<I>
where
    I: Iterator,
    I::Item: Clone + Copy + Sized,
{
    type Item = Vec<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.vec.len();
        if self.mask < (1 << n) {
            let bit_n = self.mask.count_ones() as usize;
            let mut v = Vec::with_capacity(bit_n);
            for i in 0..n {
                if self.mask >> i & 1 == 1 {
                    v.push(self.vec[i])
                }
            }
            self.mask += 1;
            Some(v)
        } else {
            None
        }
    }
}

pub trait Combinations: Iterator {
    fn combinations(self, r: usize) -> CombinationsIterator<Self>
    where
        Self: Sized,
    {
        let indices = (0..r).collect();
        CombinationsIterator {
            vec: self.collect(),
            indices,
            r,
            first: true,
        }
    }
    fn combinations_with_replacement(self, r: usize) -> CombinationsWithReplacementIterator<Self>
    where
        Self: Sized,
    {
        let indices = vec![0; r];
        CombinationsWithReplacementIterator {
            vec: self.collect(),
            indices,
            r,
            first: true,
        }
    }
}

impl<I: ?Sized> Combinations for I where I: Iterator {}
pub struct CombinationsIterator<I: Iterator> {
    vec: Vec<I::Item>,
    indices: Vec<usize>,
    r: usize,
    first: bool,
}
pub struct CombinationsWithReplacementIterator<I: Iterator> {
    vec: Vec<I::Item>,
    indices: Vec<usize>,
    r: usize,
    first: bool,
}

impl<I> Iterator for CombinationsIterator<I>
where
    I: Iterator,
    I::Item: Sized + Copy,
{
    type Item = Vec<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.vec.len();
        let r = self.r;
        if n < r {
            return None;
        }
        if self.first {
            self.first = false;
        } else {
            let mut i = r - 1;
            while self.indices[i] == i + n - r {
                if i > 0 {
                    i -= 1;
                } else {
                    return None;
                }
            }
            self.indices[i] += 1;
            for j in i + 1..r {
                self.indices[j] = self.indices[j - 1] + 1;
            }
        }
        Some(self.indices.iter().map(|&i| self.vec[i]).collect())
    }
}

impl<I> Iterator for CombinationsWithReplacementIterator<I>
where
    I: Iterator,
    I::Item: Sized + Copy,
{
    type Item = Vec<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.vec.len();
        let r = self.r;
        if self.first {
            self.first = false;
        } else {
            let mut i = r - 1;
            while self.indices[i] == n - 1 {
                if i > 0 {
                    i -= 1;
                } else {
                    return None;
                }
            }
            let v = self.indices[i];
            for j in i..r {
                self.indices[j] = v + 1;
            }
        }
        Some(self.indices.iter().map(|&i| self.vec[i]).collect())
    }
}

pub trait LexicalPermutation {
    fn prev_permutation(&mut self) -> bool;
    fn next_permutation(&mut self) -> bool;
}

impl<T: Ord> LexicalPermutation for [T] {
    fn next_permutation(&mut self) -> bool {
        if self.len() <= 1 { return false; }
        let mut i = self.len() - 1;
        while i > 0 && self[i-1] >= self[i] { i -= 1; }
        if i == 0 { return false; }
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i-1] { j -= 1; }
        self.swap(j, i-1);
        self[i..].reverse();
        true
    }
    fn prev_permutation(&mut self) -> bool {
        if self.len() <= 1 { return false; }
        let mut i = self.len() - 1;
        while i > 0 && self[i-1] <= self[i] { i -= 1; }
        if i == 0 { return false; }
        self[i..].reverse();
        let mut j = self.len() - 1;
        while j >= i && self[j-1] < self[i-1] { j -= 1; }
        self.swap(i-1, j);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accumulate() {
        let v = vec![1, 2, 3, 4, 5];
        let c = v.iter().accumulate(0, |&a, &b| a + b).collect::<Vec<_>>();
        assert_eq!(c, vec![1, 3, 6, 10, 15]);
        let c = v.iter().accumulate(1, |&a, &b| a * b).collect::<Vec<_>>();
        assert_eq!(c, vec![1, 2, 6, 24, 120]);
    }

    #[test]
    fn test_bitbrute() {
        let v = vec![2u32, 4, 6];
        let ans: Vec<Vec<&u32>> = vec![
            vec![],
            vec![&2],
            vec![&4],
            vec![&2, &4],
            vec![&6],
            vec![&2, &6],
            vec![&4, &6],
            vec![&2, &4, &6],
        ];
        for (i, comb) in v.iter().bit_brute().enumerate() {
            assert_eq!(ans[i], comb);
        }
    }

    #[test]
    fn test_bitbrute2() {
        let v = vec![1u32, 40, 1099, 1034, 5];
        let a = 1105;
        let mut f = false;
        for comb in v.iter().bit_brute() {
            let sum: u32 = comb.iter().copied().sum();
            if a == sum {
                f = true;
                assert_eq!(comb, vec![&1u32, &1099, &5]);
            }
        }
        assert!(f);
    }

    #[test]
    fn test_combination() {
        let v = vec![1, 2, 3];
        let a = vec![vec![&1, &2], vec![&1, &3], vec![&2, &3]];
        for (i, comb) in v.iter().combinations(2).enumerate() {
            assert_eq!(a[i], comb)
        }
    }

    #[test]
    fn test_combination_with_replace() {
        let v = vec![1, 2, 3];
        let a = vec![
            vec![&1, &1],
            vec![&1, &2],
            vec![&1, &3],
            vec![&2, &2],
            vec![&2, &3],
            vec![&3, &3],
        ];
        for (i, comb) in v.iter().combinations_with_replacement(2).enumerate() {
            assert_eq!(a[i], comb)
        }
    }

    #[test]
    fn test_next_permutation() {
        let mut v = vec![0, 5, 8, 10];
        assert!(v.next_permutation());
        assert_eq!(&v, &[0, 5, 10, 8]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[0, 8, 5, 10]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[0, 8, 10, 5]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[0, 10, 5, 8]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[0, 10, 8, 5]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[5, 0, 8, 10]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[5, 0, 10, 8]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[5, 8, 0, 10]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[5, 8, 10, 0]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[5, 10, 0, 8]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[5, 10, 8, 0]);
    }

    #[test]
    fn test_prev_permutation() {
        let mut v = vec![5, 10, 8, 0];
        assert!(v.prev_permutation());
        assert_eq!(&v, &[5, 10, 0, 8]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[5, 8, 10, 0]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[5, 8, 0, 10]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[5, 0, 10, 8]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[5, 0, 8, 10]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[0, 10, 8, 5]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[0, 10, 5, 8]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[0, 8, 10, 5]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[0, 8, 5, 10]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[0, 5, 10, 8]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[0, 5, 8, 10]);
        assert!(!v.prev_permutation());
    }
}
