extern crate __procon_modint as modint;

use modint::{
    Mod1012924417, Mod1224736769, Mod167772161, Mod469762049, Mod924844033, Mod998244353,
    ModuloPrimitive, StaticModInt,
};
use std::{
    iter::FromIterator,
    marker::PhantomData,
    ops::{Index, IndexMut, Mul},
};

pub type Ntt167772161 = NumberTheoreticTransform<i64, Mod167772161>;
pub type Ntt469762049 = NumberTheoreticTransform<i64, Mod469762049>;
pub type Ntt924844033 = NumberTheoreticTransform<i64, Mod924844033>;
pub type Ntt998244353 = NumberTheoreticTransform<i64, Mod998244353>;
pub type Ntt1012924417 = NumberTheoreticTransform<i64, Mod1012924417>;
pub type Ntt1224736769 = NumberTheoreticTransform<i64, Mod1224736769>;

pub struct NumberTheoreticTransform<N, M>(Vec<N>, PhantomData<fn() -> M>);

impl<N, M> From<Vec<N>> for NumberTheoreticTransform<N, M> {
    fn from(v: Vec<N>) -> Self {
        Self(v, PhantomData)
    }
}

impl<N, M> NumberTheoreticTransform<N, M> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    fn bit_reverse<T>(v: &mut [T]) {
        let mut i = 0;
        for j in 1..v.len() - 1 {
            let mut k = v.len() >> 1;
            while {
                i ^= k;
                k > i
            } {
                k >>= 1;
            }
            if i > j {
                v.swap(i, j);
            }
        }
    }
}

impl<N, M> NumberTheoreticTransform<N, M>
where
    M: ModuloPrimitive,
{
    fn dft(f: &mut [StaticModInt<M>], inv: bool) {
        let n = f.len();
        Self::bit_reverse(f);
        let pr = StaticModInt::<M>::from(M::primitive_root());
        for i in (0..).map(|i| 1 << i).take_while(|&i| i < n) {
            let mut w = pr.pow((M::modulo() - 1) / (2 * i as i64));
            if inv {
                w = 1 / w;
            }
            for k in 0..i {
                let wn = w.pow(k as i64);
                for j in (0..).map(|j| 2 * i * j).take_while(|&j| j < n) {
                    let s = f[j + k];
                    let t = f[j + k + i] * wn;
                    f[j + k] = s + t;
                    f[j + k + i] = s - t;
                }
            }
        }
        if inv {
            f.iter_mut().for_each(|v| *v /= n as i64);
        }
    }

    pub fn multiply(f: &[StaticModInt<M>], g: &[StaticModInt<M>]) -> Vec<StaticModInt<M>> {
        let m = f.len() + g.len() + 1;
        let n = m.next_power_of_two();
        let zero = StaticModInt::<M>::new(0);
        let mut ff = vec![zero; n];
        let mut gg = vec![zero; n];
        for i in 0..f.len() {
            ff[i] += f[i];
        }
        for i in 0..g.len() {
            gg[i] += g[i];
        }
        Self::dft(&mut ff, false);
        Self::dft(&mut gg, false);
        for i in 0..n {
            ff[i] *= gg[i];
        }
        Self::dft(&mut ff, true);
        ff.resize(m, zero);
        ff
    }
}

impl<N, M> Index<usize> for NumberTheoreticTransform<N, M> {
    type Output = N;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<N, M> IndexMut<usize> for NumberTheoreticTransform<N, M> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<N, M> Mul for NumberTheoreticTransform<N, M>
where
    M: ModuloPrimitive,
    N: Copy + Into<i64>,
    Vec<N>: FromIterator<i64>,
{
    type Output = Vec<N>;
    fn mul(self, rhs: Self) -> Self::Output {
        let f = self
            .0
            .iter()
            .map(|&v| StaticModInt::<M>::new(v.into()))
            .collect::<Vec<_>>();
        let g = rhs
            .0
            .iter()
            .map(|&v| StaticModInt::<M>::new(v.into()))
            .collect::<Vec<_>>();
        Self::multiply(&f, &g).iter().map(|&v| v.value()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Ntt = Ntt998244353;

    #[test]
    fn test_fft() {
        let f: Ntt = vec![0, 1, 2, 3, 4].into();
        let g: Ntt = vec![0, 1, 2, 4, 8].into();
        let x = f * g;
        assert_eq!(x, vec![0, 0, 1, 4, 11, 26, 36, 40, 32, 0, 0])
    }
}
