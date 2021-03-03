pub struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            sizes: vec![1; n],
            size: n,
        }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if x == self.parent[x] {
            x
        } else {
            let p = self.parent[x];
            self.parent[x] = self.root(p);
            self.parent[x]
        }
    }

    pub fn unite(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.root(x);
        y = self.root(y);
        if x == y {
            return false;
        }
        let (x, y) = if self.sizes[x] > self.sizes[y] {
            (x, y)
        } else {
            (y, x)
        };
        self.parent[x] = y;
        self.sizes[y] += self.sizes[x];
        self.sizes[x] = 0;
        self.size -= 1;
        true
    }

    pub fn is_same(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.root(x);
        y = self.root(y);
        x == y
    }

    pub fn group_size(&mut self, mut x: usize) -> usize {
        x = self.root(x);
        self.sizes[x]
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unionfind() {
        let mut uf = UnionFind::new(5);
        assert_eq!(uf.len(), 5);
        uf.unite(0, 1);
        assert_eq!(uf.len(), 4);
        assert_eq!(uf.root(0), uf.root(1));
        assert_ne!(uf.root(0), uf.root(2));
        assert!(uf.is_same(0, 1));
        assert!(!uf.is_same(0, 2));
        assert_eq!(uf.group_size(0), 2);
        assert_eq!(uf.group_size(4), 1);
        (2..=4).for_each(|i| {
            uf.unite(0, i);
        });
        assert_eq!(uf.group_size(0), 5);
        assert_eq!(uf.group_size(3), 5);
        assert_eq!(uf.len(), 1);
    }
}
