use std::cmp::*;
use std::collections::{btree_map::Range, BTreeMap};
use std::iter::FromIterator;
use std::ops::*;

pub struct BTreeMultiSet<T: Ord> {
    ctr: BTreeMap<T, usize>,
}

impl<T: Ord> BTreeMultiSet<T> {
    pub fn new() -> Self {
        let ctr = BTreeMap::new();
        Self { ctr }
    }

    pub fn insert(&mut self, v: T) {
        *self.ctr.entry(v).or_insert(0) += 1;
    }

    pub fn remove_one(&mut self, v: &T) -> bool {
        match self.ctr.get_mut(v) {
            Some(target) => {
                *target -= 1;
                if *target == 0 {
                    self.ctr.remove(v);
                }
                true
            }
            None => false,
        }
    }

    pub fn remove_all(&mut self, v: &T) -> usize {
        self.ctr.remove(v).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.ctr.is_empty()
    }

    pub fn count(&self, v: &T) -> usize {
        self.ctr.get(v).copied().unwrap_or(0)
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.ctr.iter().all(|(k, _)| other.count(k) == 0)
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.ctr.iter().all(|(k, &c)| c <= other.count(k))
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }

    pub fn range<R: RangeBounds<T>>(&self, range: R) -> BTreeMultiSetIterator<T> {
        let range = self.ctr.range(range);
        BTreeMultiSetIterator {
            range,
            item: None,
            count: 0,
        }
    }

    pub fn iter(&self) -> BTreeMultiSetIterator<T> {
        self.range(..)
    }
}

impl<T: Ord> Default for BTreeMultiSet<T> {
    fn default() -> BTreeMultiSet<T> {
        Self::new()
    }
}

pub struct BTreeMultiSetIterator<'a, T: Ord> {
    range: Range<'a, T, usize>,
    item: Option<&'a T>,
    count: usize,
}

impl<'a, T: Ord> Iterator for BTreeMultiSetIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            let (item, count) = self
                .range
                .next()
                .map(|(k, &v)| (Some(k), v))
                .unwrap_or((None, 0));
            self.item = item;
            self.count = count;
        }
        if self.item.is_some() {
            self.count -= 1;
        }
        self.item
    }
}

impl<'a, T: Ord> std::iter::DoubleEndedIterator for BTreeMultiSetIterator<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = if self.count == 0 {
            let (item, count) = self
                .range
                .next_back()
                .map(|(k, &v)| (Some(k), v))
                .unwrap_or((None, 0));
            self.item = item;
            self.count = count;
            item
        } else {
            self.item
        };
        if item.is_some() {
            self.count -= 1;
        }
        item
    }
}

impl<T: Ord> FromIterator<T> for BTreeMultiSet<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut set = BTreeMultiSet::new();
        for item in iter {
            set.insert(item);
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_vec() {
        let v = vec![0, 2, 1, 3, 1, 4, 2, 3, 4, 1];
        let mut u = vec![0, 1, 1, 1, 2, 2, 3, 3, 4, 4];
        let set: BTreeMultiSet<i32> = v.into_iter().collect();
        assert_eq!(set.iter().copied().collect::<Vec<_>>(), u);
        u.sort_unstable_by_key(|v| -v);
        assert_eq!(set.iter().rev().copied().collect::<Vec<_>>(), u);
    }

    #[test]
    fn test_insert_and_range() {
        let v = vec![0, 2, 1, 3, 1, 4, 2, 3, 4, 1];
        let mut set: BTreeMultiSet<i32> = v.into_iter().collect();
        set.insert(0);
        set.insert(4);
        assert_eq!(
            set.range(3..).copied().collect::<Vec<_>>(),
            vec![3, 3, 4, 4, 4]
        );
        assert_eq!(set.range(..1).copied().collect::<Vec<_>>(), vec![0, 0]);
    }

    #[test]
    fn test_remove() {
        let mut set = BTreeMultiSet::default();
        set.insert(0);
        set.insert(1);
        set.insert(1);
        set.insert(1);
        set.insert(9);
        assert_eq!(set.iter().copied().collect::<Vec<_>>(), vec![0, 1, 1, 1, 9]);
        set.remove_one(&1);
        assert_eq!(set.iter().copied().collect::<Vec<_>>(), vec![0, 1, 1, 9]);
        set.remove_all(&1);
        assert_eq!(set.iter().copied().collect::<Vec<_>>(), vec![0, 9]);
        set.remove_one(&9);
        assert_eq!(set.iter().copied().collect::<Vec<_>>(), vec![0]);
        set.remove_one(&9);
        assert_eq!(set.iter().copied().collect::<Vec<_>>(), vec![0]);
    }

    #[test]
    fn test_set_ops() {
        let s: BTreeMultiSet<i32> = vec![0, 2, 4, 6, 8].into_iter().collect();
        let t: BTreeMultiSet<i32> = vec![0, 6, 8].into_iter().collect();
        let u: BTreeMultiSet<i32> = vec![1, 3, 5, 7].into_iter().collect();
        let v = BTreeMultiSet::<i32>::new();
        assert!(v.is_empty());
        assert!(!s.is_empty());
        assert!(t.is_subset(&s));
        assert!(s.is_superset(&t));
        assert!(s.is_disjoint(&v));
        assert!(s.is_disjoint(&u));
        assert!(s.is_superset(&v));
    }
}
