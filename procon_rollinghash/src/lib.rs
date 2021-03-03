use std::{marker::PhantomData, ops::{RangeBounds, Bound}};

type Num = u128;

pub trait Hash {
    fn modulo() -> Num;
    fn base() -> Num;
}

macro_rules! define_hash {
    ($id:ident, $base:tt, $modulo:tt) => {
        #[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
        pub struct $id;
        impl Hash for $id {
            fn modulo() -> Num { $modulo }
            fn base() -> Num { $base }
        }
    }
}

define_hash!(Hash61, 1_024_578_101, 2_305_843_009_213_693_951);

pub struct RollingHash<H> {
    size: usize,
    pow: Vec<Num>,
    hash: Vec<Num>,
    __phantom: PhantomData<fn() -> H>,
}

impl<H: Hash> From<&str> for RollingHash<H> {
    fn from(s: &str) -> RollingHash<H> {
        s.as_bytes().into()
    }
}

impl<H: Hash> From<&[u8]> for RollingHash<H> {
    fn from(s: &[u8]) -> RollingHash<H> {
        let n = s.len();
        let mut pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];
        for i in 0..n {
            pow[i+1] = pow[i] * H::base() % H::modulo();
            hash[i+1] = (hash[i] * H::base() + s[i] as Num) % H::modulo();
        }
        Self { size: n, pow, hash, __phantom: PhantomData }
    }
}

impl<H: Hash> RollingHash<H> {
    pub fn hash<R: RangeBounds<usize>>(&self, range: R) -> Num {
        use Bound::*;
        let l = match range.start_bound() {
            Unbounded => 0,
            Included(&i) => i,
            Excluded(&i) => i + 1,
        };
        let r = match range.end_bound() {
            Unbounded => self.size,
            Included(&i) => i,
            Excluded(&i) => i - 1,
        };
        (self.hash[r] + H::modulo() - (self.hash[l] * self.pow[r - l]) % H::modulo()) % H::modulo()
    }
}

pub fn find_substring<H: Hash>(s: &RollingHash<H>, t: &RollingHash<H>) -> Vec<usize> {
    assert!(s.size >= t.size);
    let th = t.hash(..);
    let mut indices = vec![];
    for i in 0..=(s.size - t.size) {
        let sh = s.hash(i..=i+t.size);
        if sh == th {
            indices.push(i);
        }
    }
    indices
}

#[cfg(test)]
mod tests {
    use super::*;

    type RollingHash61 = RollingHash<Hash61>;

    #[test]
    fn test_simple_rolling_hash() {
        let s: RollingHash61 = "unvhusmjlvieloveuybouqvnqjygutqlovedkfsdfgheaiuloveaeiuvaygayfg".into();
        let t: RollingHash61 = "love".into();
        let indices = find_substring(&s, &t);
        assert_eq!(indices, vec![12, 31, 47]);
    }
}
