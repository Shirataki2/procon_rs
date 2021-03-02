pub struct SegTree<T>
where
    T: Copy,
{
    size: usize,
    data: Vec<T>,
    f: fn(&T, &T) -> T,
    id: T,
}

impl<T> SegTree<T>
where
    T: Copy,
{
    pub fn new(n: usize, f: fn(&T, &T) -> T, id: T) -> SegTree<T> {
        let mut size = 1;
        while n > size { size <<= 1; }
        let data = vec![id; 2*size];
        Self { size, data, f, id }
    }

    pub fn set(&mut self, k: usize, v: T) {
        self.data[k + self.size] = v;
    }

    pub fn get(&self, k: usize) -> T {
        self.data[k + self.size]
    }

    pub fn build(&mut self) {
        for k in (1..self.size).rev() {
            self.data[k] = (self.f)(&self.data[2 * k], &self.data[2 * k + 1]);
        }
    }

    pub fn update(&mut self, k: usize, v: T) {
        let mut k = k + self.size;
        self.data[k] = v;
        while k > 1 {
            self.data[k >> 1] = (self.f)(&self.data[k], &self.data[k^1]);
            k >>= 1;
        }
    }

    pub fn query(&self, left: usize, right: usize) -> T {
        let mut s = self.id;
        let mut l = left + self.size;
        let mut r = right + self.size;
        while l < r {
            if (l & 1) > 0 {
                s = (self.f)(&s, &self.data[l]);
                l += 1;
            }
            if (r & 1) > 0{
                s = (self.f)(&s, &self.data[r - 1]);
            }
            l >>= 1;
            r >>= 1;
        }
        s
    }
}
